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

/// Inspect metadata and list contents of ISO 9660 and UDF files.
#[derive(Parser)]
#[command(arg_required_else_help = true, long_about = libcdio_cli::HEADER, version)]
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

    /// The file argument as an option or a positional argument
    #[command(flatten)]
    pub file: FileArg,

    /// Produce only error outputs.
    #[arg(short, long)]
    pub quiet: bool,
}

#[derive(Args)]
#[group(required = true, multiple = false)]
pub struct FileArg {
    /// Path to an ISO9660 and/or UDF image
    #[arg(short = 'i', long = "input", value_name = "FILE")]
    pub option: Option<PathBuf>,

    /// Path to an ISO9660 and/or UDF image
    #[arg(value_name = "FILE")]
    pub positional: Option<PathBuf>,
}
