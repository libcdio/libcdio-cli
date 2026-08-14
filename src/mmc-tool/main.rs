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

use anyhow::{Context, Result};
use clap::Parser;
use libcdio_rs::{
    Mmc,
    mmc::{PowerCondition, RotationMode},
};
use tracing_subscriber::EnvFilter;

use crate::cli::Cli;

mod cli;

fn main() -> Result<()> {
    let cli = Cli::parse();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    let mmc = if let Some(device) = cli.device {
        Mmc::with_device(device)?
    } else {
        Mmc::new()?
    };

    if cli.actions.eject {
        mmc.allow_media_removal()?;
        mmc.eject()?;
    } else if cli.actions.close_tray {
        mmc.close_tray()?;
    } else if cli.actions.standby {
        mmc.set_power_state(PowerCondition::Standby)?;
    } else if cli.actions.mcn {
        let mcn = mmc
            .media_catalog_number()
            .context("could not get MCN")?
            .context("current media does not have a Media Catalog Number")?;
        println!("{}", mcn);
    } else if cli.actions.inquiry {
        let ident = mmc.hardware_identifiers()?;
        println!("Product: {}", ident.product);
        println!("Vendor: {}", ident.vendor);
        println!("Revision: {}", ident.revision);
    } else if let Some(speed) = cli.actions.speed {
        // some drives may not support CLV (Constant Linear Velocity)
        if mmc.set_cd_speed(RotationMode::Clv, speed, speed).is_err() {
            mmc.set_cd_speed(RotationMode::Cav, speed, speed)?;
        }
    }

    Ok(())
}
