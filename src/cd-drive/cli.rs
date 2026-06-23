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

/// Show information about a disc drive.
#[derive(Parser)]
#[command(long_about = libcdio_cli::HEADER, version)]
pub struct Cli {
    /// Show debugging information (1 = Error, 2 = Warn, 3 = Info, 4 = Debug)
    #[arg(
        default_value = "2",
        short,
        long,
        value_name = "LEVEL",
        value_parser = clap::value_parser!(u8).range(1..=4),
    )]
    pub debug: u8,

    /// Path to a disc drive.
    #[command(flatten)]
    pub drive: DriveArg,
}

#[derive(Args)]
#[group(required = false, multiple = false)]
pub struct DriveArg {
    /// Path to a disc drive.
    #[arg(short = 'i', long = "input", value_name = "DRIVE")]
    pub option: Option<PathBuf>,

    /// Path to a disc drive.
    #[arg(value_name = "DRIVE")]
    pub positional: Option<PathBuf>,
}
