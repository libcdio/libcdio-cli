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

use anyhow::Result;
use clap::Parser;
use libcdio_rs::Mmc;
use tracing_subscriber::EnvFilter;

use crate::cli::Cli;

mod cli;

fn main() -> Result<()> {
    let cli = Cli::parse();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    let _mmc = if let Some(device) = cli.device {
        Mmc::with_device(device)?
    } else {
        Mmc::new()?
    };

    Ok(())
}
