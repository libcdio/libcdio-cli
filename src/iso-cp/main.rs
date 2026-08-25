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

use std::{fs::File, io};

use anyhow::{Context, Result, bail};
use clap::Parser;
use libcdio_rs::{Iso, Udf};
use tracing_subscriber::EnvFilter;

use crate::cli::Cli;

mod cli;

fn main() -> Result<()> {
    let cli = Cli::parse();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    let image = cli.image;
    if !image.exists() {
        bail!("could not open input file at {}", image.display());
    }

    let output = cli.destination;
    let mut output = File::create(output).context("could not create output file")?;

    let iso_err = match Iso::new(image.clone()) {
        Ok(iso) => return iso9660_extract(&iso, cli.source, &mut output),
        Err(err) => err,
    };

    match Udf::new(image) {
        Ok(udf) => udf_extract(&udf, cli.source, &mut output),
        Err(udf_err) => bail!(
            "could not open file as ISO 9660 or UDF\n ISO error: {iso_err:?}\nUDF error: {udf_err:?}",
        ),
    }
}

/// Extract given file from a UDF image.
fn udf_extract(udf: &Udf, source: String, output: &mut File) -> Result<()> {
    let entry = udf.entry(source)?;

    io::copy(&mut entry.reader(), output)?;

    Ok(())
}

/// Extract given file from an ISO 9660 image.
fn iso9660_extract(iso: &Iso, source: String, output: &mut File) -> Result<()> {
    let entry = iso.entry(source)?;

    io::copy(&mut entry.reader(), output)?;

    Ok(())
}
