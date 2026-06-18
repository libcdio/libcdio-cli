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

use anyhow::{Result, bail};
use clap::Parser;
use libcdio_rs::{Iso9660, iso9660::Iso9660Extensions};

use crate::cli::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let file = cli.file.positional.or(cli.file.option).expect(
        "the cli logic must ensure that the file argument is provided either as a positional or as an option",
    );

    let extensions = Iso9660Extensions::all();
    let Some(_iso) = Iso9660::builder(&file).extensions(extensions).build() else {
        bail!("could not open ISO 9660 image");
    };

    Ok(())
}
