# libcdio-cli
Utilities to work with CD/DVD media, ISO 9660 and UDF images.
| Program    | Description                                                  |
|------------|--------------------------------------------------------------|
| drive-info | Print drive information such as identifiers and capabilities |
| iso-cp     | Copy files from an ISO 9660 or UDF filesystem.               |
| iso-ls     | List files of ISO 9660 and UDF filesystems.                  |
| mmc-cli    | Issue SCSI MMC commands to a drive.                          |

## Install
- Install [Rust][rust-install].
- Install [clang][bindgen-reqs].
- Build and install from [crates.io][libcdio-cli-cratesio]:
  ```shell
  cargo install libcdio-cli
  ```

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

Listing the contents of a UDF filesystem:
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

Listing the image metadata of an ISO 9660 filesystem:
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
