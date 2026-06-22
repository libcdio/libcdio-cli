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

use std::path::PathBuf;

use anyhow::{Context, Result, bail};
use clap::Parser;

use crate::cli::Cli;

mod cli;

fn main() -> Result<()> {
    let cli = Cli::parse();

    libcdio_cli::setup_logs(cli.debug);

    let image = cli.image.positional.or(cli.image.option)
        .expect( "the cli logic must ensure that the file argument is provided either as a positional or as an option");

    if !image.exists() {
        bail!("could not open input file at {}", image.display());
    }

    let output_default = cli
        .extract
        .file_name()
        .with_context(|| format!("invalid extract file name: {}", cli.extract.display()))?;
    let _output = cli.output_file.unwrap_or(PathBuf::from(output_default));

    Ok(())
}
