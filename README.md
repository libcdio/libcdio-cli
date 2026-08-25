# libcdio-cli
Utilities to work with CD/DVD media, ISO 9660 and UDF images.
| Program    | Description                                                    |
|------------|----------------------------------------------------------------|
| drive-info | Prints drive information such as identifiers and capabilities. |
| iso-cp     | Copies files from ISO 9660 and UDF filesystems.                |
| iso-ls     | Lists files of ISO 9660 and UDF filesystems.                   |
| mmc-cli    | Issues SCSI MMC commands to a disc drive.                      |


## Install
- Install [Rust][rust-install].
- Install [clang][bindgen-reqs].
- Build and install from [crates.io][libcdio-cli-cratesio]:
  ```shell
  cargo install libcdio-cli
  ```

## drive-info
Prints drive information such as identifiers and capabilities.
```console
$ drive-info -h
Show information about a disc drive

Usage: drive-info [OPTIONS] [DRIVE]

Arguments:
  [DRIVE]  Path to a disc drive

Options:
  -i, --input <DRIVE>  Path to a disc drive
  -h, --help           Print help (see more with '--help')
  -V, --version        Print version
```

<details>
<summary>Example: Output from a Verbatim brand CD/DVD drive.</summary>

```console
$ drive-info
Using drive /dev/cdrom
Device information:
   Vendor   : MATSHITA
   Model    : DVD-RAM UJ8C0
   Revision : SB02
MMC information:
   Supported features:
      [*] Profile List:
         [ ] DVD-RAM
         [ ] DVD+R Double Layer
         [ ] DVD+R
         [ ] DVD+RW
         [ ] DVD-RW Restricted Overwrite
         [ ] DVD-RW Sequential Recording
         [ ] DVD-R Dual Layer Jump Recording
         [ ] DVD-R Dual Layer Sequential recording
         [ ] DVD-R Sequential Recording
         [ ] DVD-ROM
         [ ] CD-RW
         [ ] CD-R
         [ ] CD-ROM
         [ ] Removable disk
      [*] Core:
         + Interface: Serial ATAPI
      [*] Morphing:
         - Asynchronous events
         + Operational change events
      [*] Removable Medium:
         + Eject
         + Lock
         + Prevent Jumper
         + Loading Mechanism: Tray type
      [ ] CD Read:
         + C2 Errors
         + CD-Text
         - DAP
      [ ] CD Audio External Play:
         + SCAN
         + Separate Channel Mute
         + Separate Volume
         + Volume Levels: 256
      [ ] DVD-CSS:
      [*] Drive Serial Number:
         + S/N: HP61  188576
Drive capabilities:
   Hardware:
      + Close Tray
      + Eject
      + Lock
      + Multi Session
      + Hard Reset
   Read:
      + Play Audio
      + CD-DA
      + CD-R
      + CD-RW
      + DVD-ROM
      + C2 Errors
      + Mode 2 Form 1 (VCD)
      + Mode 2 Form 2 (VCD)
      + MCN
      + ISRC
   Write:
      + CD-R
      + CD-RW
      + DVD-R
      + DVD-RAM

Using drive /dev/sr0
Device information:
   Vendor   : MATSHITA
   Model    : DVD-RAM UJ8C0
   Revision : SB02
MMC information:
   Supported features:
      [*] Profile List:
         [ ] DVD-RAM
         [ ] DVD+R Double Layer
         [ ] DVD+R
         [ ] DVD+RW
         [ ] DVD-RW Restricted Overwrite
         [ ] DVD-RW Sequential Recording
         [ ] DVD-R Dual Layer Jump Recording
         [ ] DVD-R Dual Layer Sequential recording
         [ ] DVD-R Sequential Recording
         [ ] DVD-ROM
         [ ] CD-RW
         [ ] CD-R
         [ ] CD-ROM
         [ ] Removable disk
      [*] Core:
         + Interface: Serial ATAPI
      [*] Morphing:
         - Asynchronous events
         + Operational change events
      [*] Removable Medium:
         + Eject
         + Lock
         + Prevent Jumper
         + Loading Mechanism: Tray type
      [ ] CD Read:
         + C2 Errors
         + CD-Text
         - DAP
      [ ] CD Audio External Play:
         + SCAN
         + Separate Channel Mute
         + Separate Volume
         + Volume Levels: 256
      [ ] DVD-CSS:
      [*] Drive Serial Number:
         + S/N: HP61  188576
Drive capabilities:
   Hardware:
      + Close Tray
      + Eject
      + Lock
      + Multi Session
      + Hard Reset
   Read:
      + Play Audio
      + CD-DA
      + CD-R
      + CD-RW
      + DVD-ROM
      + C2 Errors
      + Mode 2 Form 1 (VCD)
      + Mode 2 Form 2 (VCD)
      + MCN
      + ISRC
   Write:
      + CD-R
      + CD-RW
      + DVD-R
      + DVD-RAM
```
</details>

## iso-cp
Copies files from ISO 9660 or UDF filesystem.
```console
$ iso-cp -h
Copy files from an ISO 9660 or UDF filesystem

Usage: iso-cp <IMAGE> <SOURCE> <DESTINATION>

Arguments:
  <IMAGE>        Path to an ISO 9660 or UDF image
  <SOURCE>       Path to a source file in the image
  <DESTINATION>  Path to a destination file or directory

Options:
  -h, --help     Print help (see more with '--help')
  -V, --version  Print version
```

<details>
<summary>Example: Copying a file from a UDF filesystem:</summary>

```console
$ iso-cp tests/data/udf1.iso licenses/COPYING.LESSER ./lgpl
$ cat lgpl | head -2
                   GNU LESSER GENERAL PUBLIC LICENSE
                       Version 3, 29 June 2007
```
</details>

Copying whole directories is currently not supported.

## iso-ls
Lists files of an ISO 9660 or UDF filesystem.
```console
$ iso-ls -h
Inspect metadata and list contents of ISO 9660 and UDF files

Usage: iso-ls [OPTIONS] <IMAGE>

Arguments:
  <IMAGE>  Path to an ISO 9660 or UDF image

Options:
  -m, --metadata  Print image metadata
  -h, --help      Print help (see more with '--help')
  -V, --version   Print version
```

<details>
<summary>Example: Listing the contents of a UDF filesystem:</summary>

```console
$ iso-ls tests/data/udf1.iso
/:
  dr-xr-xr-x 2000 3000   2        88 Jun 19 2026 20:42:57 .
  dr-xr-xr-x 2000 3000   1       144 Jun 19 2026 20:42:57 licenses

/licenses/:
  dr-xr-xr-x 2000 3000   2        88 Jun 19 2026 20:42:57 .
  -r--r--r-- 2000 3000   1     35149 Jun 19 2026 20:41:12 COPYING
  -r--r--r-- 2000 3000   1      7652 Jun 19 2026 20:41:16 COPYING.LESSER
```
</details>

<details>
<summary>Example: Listing the image metadata of an ISO 9660 filesystem:</summary>

```console
$ iso-ls -m tests/data/joliet.iso
Image       : tests/data/joliet.iso
Application : K3B THE CD KREATOR VERSION 0.11.12 (C) 2003 SEBASTIAN TRUEG AND THE K3B TEAM
Preparer    : K3b - Version 0.11.12
Publisher   : Rocky Bernstein
System      : LINUX
Volume      : K3b data project
Joliet      : Level 3
Rock Ridge  : no
```
</details>

## mmc-cli
Issues SCSI MMC commands to a disc drive.
```console
$ mmc-cli -h
Usage: mmc-cli <--eject|--close-tray|--standby|--mcn|--inquiry|--speed <SPEED>> [DEVICE]

Arguments:
  [DEVICE]  Path to an MMC device

Options:
  -e, --eject          Eject the drive
  -c, --close-tray     Close the tray, if present
  -s, --standby        Put the device into standby
  -m, --mcn            Get the MCN (Media Catalog Number) of the media
  -i, --inquiry        Get hardware identifiers (Product, Vendor and Revision)
  -S, --speed <SPEED>  Set the drive read and write speed in KB/s
  -h, --help           Print help (see more with '--help')
  -V, --version        Print version
```

<details>
<summary>Example: Get hardware identifiers of a drive:</summary>

```console
$ mmc-cli -i
Product: DVD-RAM UJ8C0
Vendor: MATSHITA
Revision: SB02
```
</details>


## Development
### Use the provided Git Hooks
These are set to perform lint and formatting checks before every
commit:
```sh
git config core.hooksPath .githooks
```

If you have to skip hooks for a draft commit, use `--no-verify`:
```sh
git commit --no-verify
```

### Tests
Use `cargo test` to run the tests.

## See also
These programs are based on the functionality provided by
[libcdio-rs][libcdio-rs-cratesio].

## License
Copyright (C) 2026 Shiva Kiran Koninty <shiva@skran.xyz>

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by the
Free Software Foundation, either version 3 of the License, or (at your
option) any later version.

This program is distributed in the hope that it will be useful, but
WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program. If not, see <https://www.gnu.org/licenses/>.

[rust-install]: https://rust-lang.org/tools/install/
[bindgen-reqs]: https://rust-lang.github.io/rust-bindgen/requirements.html
[libcdio-cli-cratesio]: https://crates.io/crates/libcdio-cli
[libcdio-rs-cratesio]: https://crates.io/crates/libcdio-rs
