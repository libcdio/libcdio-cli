use std::fs;

use assert_cmd::{Command, cargo::cargo_bin_cmd};
use assert_fs::{NamedTempFile, assert::PathAssert};

fn cmd() -> Command {
    cargo_bin_cmd!("iso-cp")
}

static UDF_FILE: &str = "tests/data/udf1.iso";
#[test]
fn extract_udf() {
    let output = NamedTempFile::new("out").unwrap();
    cmd()
        .arg(UDF_FILE)
        .arg("licenses/COPYING")
        .arg(output.path())
        .assert()
        .success();

    let gpl = fs::read_to_string("COPYING").unwrap();
    output.assert(gpl);
}

static ISO9660_FILE: &str = "tests/data/xa.iso";
#[test]
fn extract_iso9660() {
    let output = NamedTempFile::new("out").unwrap();
    cmd()
        .arg(ISO9660_FILE)
        .arg("copying")
        .arg(output.path())
        .assert()
        .success();

    let gpl = fs::read_to_string("COPYING").unwrap();
    output.assert(gpl);
}
