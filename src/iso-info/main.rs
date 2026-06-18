// Copyright (C) 2026 Shiva Kiran Koninty <shiva@skran.xyz>
//
// This file is part of libcdio-cli.
//
// libcdio-cli is free software: you can redistribute it and/or
// modify it under the terms of the GNU General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// libcdio-cli is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with libcdio-cli. If not, see <https://www.gnu.org/licenses/>.

mod cli;

use std::{
    collections::VecDeque,
    io,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result, bail};
use clap::Parser;
use libcdio_rs::{
    Iso9660,
    iso9660::{Iso9660Extensions, xa::XaFileAttributes},
};
use time::{UtcOffset, format_description::BorrowedFormatItem, macros::format_description};

use crate::cli::Cli;

const DATE_FMT: &[BorrowedFormatItem] =
    format_description!("[month repr:short] [day] [year] [hour]:[minute]:[second]");
static LINE: &str = "__________________________________";

fn main() -> Result<()> {
    let cli = Cli::parse();

    libcdio_cli::setup_logs(cli.debug);
    let mut output: &mut dyn io::Write = if cli.quiet {
        &mut io::sink()
    } else {
        &mut io::stdout()
    };

    let file = cli.file.positional.or(cli.file.option).expect(
        "the cli logic must ensure that the file argument is provided either as a positional or as an option",
    );

    let mut extensions = Iso9660Extensions::all();
    if cli.no_joliet {
        extensions -= Iso9660Extensions::JolietLevel1;
        extensions -= Iso9660Extensions::JolietLevel2;
        extensions -= Iso9660Extensions::JolietLevel3;
    }

    let Some(iso) = Iso9660::builder(&file).extensions(extensions).build() else {
        bail!("error opening iso9660 image: {}", file.display());
    };

    print_iso9660_metadata(&iso, &file, &mut output)
        .context("io error while printing iso9660 metadata")?;

    if cli.show_rock_ridge.is_some() {
        let file_limit = cli.show_rock_ridge.filter(|file_limit| *file_limit != 0);
        print_rock_ridge(&iso, file_limit, &mut output)
            .context("io error while printing rock ridge status")?;
    }

    print_joliet_level(&iso, &mut output).context("io error while printing joliet level")?;

    if cli.iso9660 {
        print_iso9660_contents(&iso, &mut output, !cli.no_rock_ridge, !cli.no_xa)
            .context("error printing iso9660 contents")?;
    }

    Ok(())
}

fn print_iso9660_metadata(
    iso: &Iso9660,
    path: &Path,
    mut out: impl io::Write,
) -> Result<(), io::Error> {
    writeln!(out, "{LINE}")?;
    writeln!(out, "ISO 9660 image: {}", path.display())?;
    let mut write_if_some = |key, val| {
        let Some(val) = val else { return Ok(()) };
        writeln!(out, "{key} : {val}")
    };
    write_if_some("Application", iso.application())?;
    write_if_some("Preparer   ", iso.data_preparer())?;
    write_if_some("Publisher  ", iso.publisher())?;
    write_if_some("System     ", iso.system())?;
    write_if_some("Volume     ", iso.volume())?;
    write_if_some("Volume Set ", iso.volume_set())?;

    Ok(())
}

fn print_rock_ridge(
    iso: &Iso9660,
    file_limit: Option<u64>,
    mut out: impl io::Write,
) -> Result<(), io::Error> {
    let status = match iso.have_rock_ridge(file_limit) {
        Some(true) => "yes",
        Some(false) => "no",
        None => "possibly not",
    };
    writeln!(out, "Rock Ridge  : {}", status)
}

/// Outputs the file contents of the ISO 9660 image in an ls-like listing format.
fn print_iso9660_contents(
    iso: &Iso9660,
    mut out: impl io::Write,
    use_rock_ridge: bool,
    use_xa: bool,
) -> Result<()> {
    const ISO9660_DEPTH_LIMIT: usize = 512;
    let mut dirs = VecDeque::new();
    dirs.push_back((PathBuf::from("/"), 0)); // (path, depth)

    writeln!(out, "{}", LINE)?;
    writeln!(out, "ISO-9660 Information")?;

    while let Some((dir_path, depth)) = dirs.pop_front() {
        if depth == ISO9660_DEPTH_LIMIT {
            bail!("directory recursion too deep. ISO most probably damaged");
        }

        writeln!(out, "{}:", dir_path.display())?;

        for entry in iso
            .read_dir(&dir_path)
            .with_context(|| format!("could not read iso: {}", dir_path.display()))?
        {
            let rock_ridge = use_rock_ridge.then_some(entry.rock_ridge()).flatten();
            let translated_name = entry.filename();
            let entry_name = if rock_ridge.is_none() {
                translated_name.as_deref()
            } else {
                entry.filename_raw()
            }
            .with_context(|| format!("could not get file name of lsn: {}", entry.lsn()))?;

            let full_path = dir_path.join(entry_name);
            if entry.is_dir() && entry_name != "." && entry_name != ".." {
                // .join("") adds a trailing slash
                dirs.push_back((full_path.join(""), depth + 1));
            }

            write!(out, " ")?;
            if let Some(rock) = &rock_ridge {
                let total_size = if let Some(symlink) = &rock.symlink_to {
                    symlink.len() as u64
                } else {
                    entry.total_size()
                };
                write!(out, " {}", rock.mode)?;
                write!(out, " {:3}", rock.hard_links)?;
                write!(out, " {}", rock.user_id)?;
                write!(out, " {}", rock.group_id)?;
                write!(out, " [LSN {:6}]", entry.lsn())?;
                write!(out, " {:9}", total_size)?;
            } else if use_xa && let Some(xa) = entry.xa() {
                write!(out, " {}", xa_file_mode_str(xa.file_attr))?;
                write!(out, " {}", xa.user_id)?;
                write!(out, " {}", xa.group_id)?;
                write!(out, " [fn {:02}]", xa.file_num)?;
                write!(out, " [LSN {:6}]", entry.lsn())?;
                if let Some(m2f2_size) = xa.mode2form2_size() {
                    write!(out, " {:9}", m2f2_size)?;
                    write!(out, " ({:9})", entry.total_size())?;
                } else {
                    write!(out, " {:9}", entry.total_size())?;
                }
            } else {
                write!(out, " {}", if entry.is_dir() { 'd' } else { '-' })?;
                write!(out, " [LSN {:6}]", entry.lsn())?;
                write!(out, " {:9}", entry.total_size())?;
            }

            let time = if let Some(rock) = &rock_ridge
                && let Some(mtime) = rock.modify_time
            {
                mtime
            } else {
                entry
                    .timestamp()
                    .with_context(|| format!("got invalid timestamp: {}", full_path.display()))?
            };

            let local = UtcOffset::current_local_offset()
                .context("could not get current time offset from system")?;
            let time = time
                .to_offset(local)
                .format(DATE_FMT)
                .with_context(|| format!("could not format timestamp: {}", full_path.display()))?;

            write!(out, " {}", time)?;
            write!(out, " {}", entry_name)?;

            if let Some(rock) = rock_ridge
                && let Some(symlink_to) = rock.symlink_to
            {
                write!(out, " -> {}", symlink_to)?;
            }

            writeln!(out)?;
        }

        writeln!(out)?;
    }

    Ok(())
}

/// Returns an ls-like string representation of the XA file mode attributes.
/// Example: "d---1xrxr-r"
fn xa_file_mode_str(attr: XaFileAttributes) -> String {
    let mut mode_str = String::new();
    let mut has_attr = |attribute, letter| {
        if attr.contains(attribute) {
            mode_str.push(letter)
        } else {
            mode_str.push('-')
        }
    };
    has_attr(XaFileAttributes::Directory, 'd');
    has_attr(XaFileAttributes::Cdda, 'a');
    has_attr(XaFileAttributes::Interleaved, 'i');
    has_attr(XaFileAttributes::Mode2Form2, '2');
    has_attr(XaFileAttributes::Mode2, '1');
    has_attr(XaFileAttributes::OwnerExecute, 'x');
    has_attr(XaFileAttributes::OwnerRead, 'r');
    has_attr(XaFileAttributes::GroupExecute, 'x');
    has_attr(XaFileAttributes::GroupRead, 'r');
    has_attr(XaFileAttributes::WorldExecute, 'x');
    has_attr(XaFileAttributes::WorldRead, 'r');

    mode_str
}

fn print_joliet_level(iso: &Iso9660, mut out: impl io::Write) -> Result<(), io::Error> {
    let Some(joliet_level) = iso.joliet_level() else {
        return writeln!(out, "No Joliet extensions");
    };

    writeln!(out, "Joliet Level: {}", u8::from(joliet_level))
}
