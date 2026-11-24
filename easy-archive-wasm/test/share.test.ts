import { expect, test } from "vitest";
import { humanSize, isSafeLink, modeToString } from "../src-ts/share";

test("modeToString", () => {
  expect(modeToString(0o755, true)).toEqual("drwxr-xr-x");
  expect(modeToString(0o644, false)).toEqual("-rw-r--r--");
});

test("humanSize", () => {
  expect(humanSize(0)).toEqual("0");
  expect(humanSize(1024)).toEqual("1K");
});

/**
 * Links are only created when they resolve inside the extraction root; this
 * module is also bundled for the browser, so the path logic is self-contained
 * rather than relying on `node:path`.
 */
test("isSafeLink", () => {
  const cases: [root: string, path: string, target: string, safe: boolean][] = [
    // Same directory.
    ["/tmp/out", "/tmp/out/lib/libfoo.so.1", "libfoo.so.1.2.3", true],
    // Up and back down, still inside.
    ["/tmp/out", "/tmp/out/docs/libdir", "../lib", true],
    // Leaves the root.
    ["/tmp/out", "/tmp/out/pack/escape", "../../secret", false],
    // Absolute targets cannot be checked against the root.
    ["/tmp/out", "/tmp/out/abs", "/etc", false],
    // A trailing separator on the root changes nothing.
    ["/tmp/out/", "/tmp/out/a", "b", true],
    // Windows paths are handled the same way.
    ["C:\\tmp\\out", "C:\\tmp\\out\\lib\\a", "b", true],
    ["C:\\tmp\\out", "C:\\tmp\\out\\pack\\a", "..\\..\\secret", false],
    ["C:\\tmp\\out", "C:\\tmp\\out\\a", "D:\\other", false],
  ];

  for (const [root, path, target, safe] of cases) {
    expect(isSafeLink(root, path, target), `${root}: ${path} -> ${target}`)
      .toEqual(safe);
  }
});
