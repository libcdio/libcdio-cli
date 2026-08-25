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

use clap::Parser;

/// Inspect metadata and list contents of ISO 9660 and UDF files.
#[derive(Parser)]
#[command(arg_required_else_help = true, long_about = libcdio_cli::HEADER, version)]
pub struct Cli {
    /// Path to an ISO 9660 or UDF image.
    #[arg(value_name = "IMAGE")]
    pub image: PathBuf,

    /// Print image metadata.
    #[arg(short, long)]
    pub metadata: bool,

    /// Produce only error outputs.
    #[arg(short, long)]
    pub quiet: bool,
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    use super::*;

    #[test]
    fn verify_cli() {
        Cli::command().debug_assert();
    }
}
