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

use std::{io, path::Path};

use anyhow::{Context, Result, bail};
use clap::Parser;
use libcdio_rs::{Iso9660, iso9660::Iso9660Extensions};

use crate::cli::Cli;

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

fn print_joliet_level(iso: &Iso9660, mut out: impl io::Write) -> Result<(), io::Error> {
    let Some(joliet_level) = iso.joliet_level() else {
        return writeln!(out, "No Joliet extensions");
    };

    writeln!(out, "Joliet Level: {}", u8::from(joliet_level))
}
