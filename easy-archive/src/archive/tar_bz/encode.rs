/// TAR.BZ2 encoding implementation
use crate::{
    File,
    error::{ArchiveError, Result},
    traits::Encode,
};

use super::TarBz;

impl Encode for TarBz {
    fn encode(_files: Vec<File>) -> Result<Vec<u8>> {
        Err(ArchiveError::UnsupportedFormat(
            "TAR.BZ2 encoding is not supported (bzip2-rs lacks encoder)".to_string(),
        ))
    }
}
