/// Command-line interface for easy-archive
///
/// This binary provides a simple CLI for compressing and decompressing archives.
use easy_archive::Fmt;
use easy_archive::cli::{
    get_default_output, get_help_text,
    handle_compression, handle_decompression,
};
#[cfg(feature = "convert")]
use easy_archive::cli::display_error;
#[cfg(feature = "convert")]
use easy_archive::convert::convert_archive_file;

use std::process;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, after_help = get_help_text())]
struct Cli {
    /// Input files or directories (multiple allowed)
    #[arg(required = true)]
    inputs: Vec<String>,

    /// Output archive or directory
    #[arg(short, long)]
    output: Option<String>,
}

/// Handle format conversion (transcoding) between archive formats
///
/// Available only when the `convert` feature is enabled (requires both
/// `encode` and `decode` features).
#[cfg(feature = "convert")]
fn handle_convert(input: &str, output: &str, input_fmt: Fmt, output_fmt: Fmt) {
    match convert_archive_file(input, output, input_fmt, output_fmt) {
        Ok(()) => {
            println!(
                "Converted {} from {:?} to {:?}",
                input, input_fmt, output_fmt
            );
            println!("Output written to {}", output);
        }
        Err(e) => {
            display_error(&e);
            process::exit(1);
        }
    }
}

/// Command-line interface for easy-archive
///
/// This binary provides a simple CLI for compressing and decompressing archives.
fn main() {
    let cli = Cli::parse();
    let inputs = cli.inputs;

    let input_fmt = if inputs.len() == 1 {
        Fmt::guess(&inputs[0])
    } else {
        None // Multiple files always evaluate to a single compression output archive
    };

    let output = cli
        .output
        .unwrap_or_else(|| get_default_output(&inputs, input_fmt));

    let output_fmt = Fmt::guess(&output);

    // Handle compression or decompression based on enabled features
    match (input_fmt, output_fmt) {
        #[cfg(feature = "decode")]
        (Some(fmt), None) => {
            // Decompression
            handle_decompression(&inputs[0], &output, fmt);
        }
        #[cfg(feature = "encode")]
        (None, Some(fmt)) => {
            // Compression
            handle_compression(&inputs, &output, fmt);
        }
        #[cfg(feature = "convert")]
        (Some(input_fmt), Some(output_fmt)) => {
            // Format conversion (transcoding)
            handle_convert(&inputs[0], &output, input_fmt, output_fmt);
        }
        #[cfg(not(feature = "convert"))]
        (Some(_), Some(_)) => {
            eprintln!("Error: Both input and output are archive formats.");
            eprintln!("Please specify one as a directory for compression/decompression.");
            eprintln!("Enable the 'convert' feature to convert between formats directly.");
            process::exit(1);
        }
        #[cfg(all(feature = "decode", not(feature = "encode")))]
        (None, Some(_)) => {
            eprintln!("Error: Encode operation is not enabled.");
            eprintln!("This binary was compiled with decode-only support.");
            process::exit(1);
        }
        #[cfg(all(not(feature = "decode"), feature = "encode"))]
        (Some(_), None) => {
            eprintln!("Error: Decode operation is not enabled.");
            eprintln!("This binary was compiled with encode-only support.");
            process::exit(1);
        }
        (None, None) => {
            #[cfg(all(not(feature = "decode"), not(feature = "encode")))]
            {
                eprintln!("Error: No operations enabled.");
                eprintln!("Please enable 'encode' or 'decode' feature.");
            }
            #[cfg(any(feature = "decode", feature = "encode"))]
            {
                eprintln!("Error: Cannot identify input and output formats.");
                eprintln!("At least one must be a recognized archive format.");
            }
            process::exit(1);
        }
    }
}
