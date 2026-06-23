use clap::Parser;

use crate::cli::Cli;

mod cli;

fn main() {
    let cli = Cli::parse();

    libcdio_cli::setup_logs(cli.debug);

    let _drive = cli.drive.positional.or(cli.drive.option);
}
