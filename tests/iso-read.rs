use std::fs;

use assert_cmd::{Command, cargo::cargo_bin_cmd};
use assert_fs::{NamedTempFile, assert::PathAssert};

fn cmd() -> Command {
    cargo_bin_cmd!("iso-read-rs")
}

static UDF_FILE: &str = "../test-data/udf1.iso";
#[test]
fn extract_udf() {
    let output = NamedTempFile::new("out").unwrap();
    cmd()
        .arg("-e")
        .arg("licenses/COPYING")
        .arg("-i")
        .arg(UDF_FILE)
        .arg("-o")
        .arg(output.path())
        .arg("-U")
        .assert()
        .success();

    let gpl = fs::read_to_string("../COPYING").unwrap();
    output.assert(gpl);
}

static ISO9660_FILE: &str = "../test-data/xa.iso";
#[test]
fn extract_iso9660() {
    let output = NamedTempFile::new("out").unwrap();
    cmd()
        .arg("-e")
        .arg("copying")
        .arg("-i")
        .arg(ISO9660_FILE)
        .arg("-o")
        .arg(output.path())
        .assert()
        .success();

    let gpl = fs::read_to_string("../COPYING").unwrap();
    output.assert(gpl);
}
