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

use clap::{Args, Parser};

#[derive(Debug, Parser)]
#[command(arg_required_else_help = true, long_about = libcdio_cli::HEADER, version)]
pub struct Cli {
    /// Path to an MMC device.
    pub device: Option<PathBuf>,

    #[command(flatten)]
    pub actions: MmcActions,
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
pub struct MmcActions {
    /// Eject the drive
    #[arg(short, long)]
    pub eject: bool,

    /// Close the tray, if present
    #[arg(short, long)]
    pub close_tray: bool,

    /// Put the device into standby
    #[arg(short, long)]
    pub standby: bool,

    /// Get the MCN (Media Catalog Number) of the media
    #[arg(short, long)]
    pub mcn: bool,

    /// Get hardware identifiers (Product, Vendor and Revision)
    #[arg(short, long)]
    pub inquiry: bool,

    /// Set the drive read and write speed in KB/s.
    ///
    /// Falls back to the nearest supported value if the provided value is not
    /// supported.
    #[arg(short = 'S', long)]
    pub speed: Option<u16>,
}
