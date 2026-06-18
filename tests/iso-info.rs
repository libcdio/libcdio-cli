use assert_cmd::{Command, cargo::cargo_bin_cmd};

fn cmd() -> Command {
    cargo_bin_cmd!("iso-info")
}

static ROCK_RIDGE_FILE: &str = "../test-data/rock-ridge.iso";
static ROCK_METADATA: &str = r"__________________________________
ISO 9660 image: ../test-data/rock-ridge.iso
Application : K3B THE CD KREATOR VERSION 0.11.20 (C) 2003 SEBASTIAN TRUEG AND THE K3B TEAM
Preparer    : K3b - Version 0.11.20
Publisher   : Rocky Bernstein
System      : LINUX
Volume      : Rock Ridge Copy test
No Joliet extensions
";
#[test]
fn rock_metadata() {
    cmd()
        .arg(ROCK_RIDGE_FILE)
        .assert()
        .success()
        .stdout(ROCK_METADATA);
}

static JOLIET_FILE: &str = "../test-data/joliet.iso";
static JOLIET_METADATA: &str = r"__________________________________
ISO 9660 image: ../test-data/joliet.iso
Application : K3B THE CD KREATOR VERSION 0.11.12 (C) 2003 SEBASTIAN TRUEG AND THE K3B TEAM
Preparer    : K3b - Version 0.11.12
Publisher   : Rocky Bernstein
System      : LINUX
Volume      : K3b data project
Joliet Level: 3
";
#[test]
fn joliet_metadata() {
    cmd()
        .arg(JOLIET_FILE)
        .assert()
        .success()
        .stdout(JOLIET_METADATA);
}

static XA_FILE: &str = "../test-data/xa.iso";
static XA_METADATA: &str = r"__________________________________
ISO 9660 image: ../test-data/xa.iso
Application : GENISOIMAGE ISO 9660/HFS FILESYSTEM CREATOR (C) 1993 E.YOUNGDALE (C) 1997-2006 J.PEARSON/J.SCHILLING (C) 2006-2007 CDRKIT TEAM
System      : LINUX
Volume      : CDROM
No Joliet extensions
";
#[test]
fn xa_metadata() {
    cmd().arg(XA_FILE).assert().success().stdout(XA_METADATA);
}

static ROCK_CONTENTS: &str = r"__________________________________
ISO-9660 Information
/:
  dr-xr-xr-x   4 0 0 [LSN     23]      2048 Oct 22 2004 02:21:14 .
  dr-xr-xr-x   2 0 0 [LSN     23]      2048 Oct 22 2004 02:21:14 ..
  dr-xr-xr-x   2 0 0 [LSN     24]      2048 Mar 05 2005 16:12:25 copy
  lr-xr-xr-x   1 0 0 [LSN     27]         7 Mar 05 2005 15:26:00 Copy2 -> COPYING
  -r--r--r--   1 0 0 [LSN     27]     17992 Mar 05 2005 15:25:51 COPYING
  br--r--r--   1 0 0 [LSN     36]         0 Mar 05 2005 15:32:05 fd0
  dr-xr-xr-x   2 0 0 [LSN     25]      2048 Mar 05 2005 16:12:25 tmp
  cr--r--r--   1 0 0 [LSN     36]         0 Mar 05 2005 15:31:42 zero

/copy/:
  dr-xr-xr-x   2 0 0 [LSN     24]      2048 Mar 05 2005 16:12:25 .
  dr-xr-xr-x   4 0 0 [LSN     23]      2048 Mar 05 2005 16:12:25 ..
  lr-xr-xr-x   1 0 0 [LSN     36]        10 Mar 05 2005 15:27:05 COPYING -> ../COPYING

/tmp/:
  dr-xr-xr-x   2 0 0 [LSN     25]      2048 Mar 05 2005 16:12:25 .
  dr-xr-xr-x   4 0 0 [LSN     23]      2048 Mar 05 2005 16:12:25 ..
  lr-xr-xr-x   1 0 0 [LSN     36]        18 Mar 05 2005 15:51:05 COPYING -> ../copying/COPYING

";
#[test]
fn rock_contents() {
    cmd()
        .env("TZ", "UTC")
        .arg("-l")
        .arg(ROCK_RIDGE_FILE)
        .assert()
        .success()
        .stdout(ROCK_METADATA.to_owned() + ROCK_CONTENTS);
}

static JOLIET_CONTENTS: &str = r"__________________________________
ISO-9660 Information
/:
  d [LSN     31]      2048 Oct 22 2004 22:44:59 .
  d [LSN     31]      2048 Oct 22 2004 22:44:59 ..
  d [LSN     32]      2048 Oct 22 2004 22:44:59 libcdio

/libcdio/:
  d [LSN     32]      2048 Oct 22 2004 22:44:59 .
  d [LSN     31]      2048 Oct 22 2004 22:44:59 ..
  - [LSN     34]     17992 Mar 12 2004 07:18:03 COPYING
  - [LSN     43]      2156 Jun 26 2004 10:01:09 README
  - [LSN     45]      2849 Aug 12 2004 09:22:23 README.libcdio
  d [LSN     33]      2048 Oct 22 2004 22:44:59 test

/libcdio/test/:
  d [LSN     33]      2048 Oct 22 2004 22:44:59 .
  d [LSN     32]      2048 Oct 22 2004 22:44:59 ..
  - [LSN     47]        74 Jul 25 2004 09:52:32 isofs-m1.cue

";
#[test]
fn joliet_contents() {
    cmd()
        .env("TZ", "UTC")
        .arg("-l")
        .arg(JOLIET_FILE)
        .assert()
        .success()
        .stdout(JOLIET_METADATA.to_owned() + JOLIET_CONTENTS);
}

static XA_CONTENTS: &str = r"__________________________________
ISO-9660 Information
/:
  d---1xrxrxr 1000 3000 [fn 00] [LSN     23]      2048 Jun 08 2026 04:44:35 .
  d---1xrxrxr 1000 3000 [fn 00] [LSN     23]      2048 Jun 08 2026 04:44:35 ..
  ----1--xr-- 1000 3000 [fn 00] [LSN     25]     35149 Jun 08 2026 04:43:19 copying

";
#[test]
fn xa_contents() {
    cmd()
        .env("TZ", "UTC")
        .arg("-l")
        .arg(XA_FILE)
        .assert()
        .success()
        .stdout(XA_METADATA.to_owned() + XA_CONTENTS);
}

static UDF_FILE: &str = "../test-data/udf.iso";
static UDF_OUTPUT: &str = "/:
  dr-xr-xr-x 4294967295 4294967295   1       100 Feb 20 2014 01:26:20 .
  -r-xr-xr-x 4294967295 4294967295   1        10 Feb 20 2014 01:25:12 FéжΘvrier

";
#[test]
fn udf() {
    cmd()
        .env("TZ", "UTC")
        .arg("-U")
        .arg(UDF_FILE)
        .arg("--debug")
        .arg("1")
        .assert()
        .success()
        .stdout(UDF_OUTPUT);
}
