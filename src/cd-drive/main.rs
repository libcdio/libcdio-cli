use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use libcdio_rs::Drive;

use crate::cli::Cli;

mod cli;

fn main() -> Result<()> {
    let cli = Cli::parse();

    libcdio_cli::setup_logs(cli.debug);

    let drive = cli.drive.positional.or(cli.drive.option);

    if let Some(drive) = drive {
        print_drive_info(drive)?;
    } else if let drives = Drive::drives()
        && !drives.is_empty()
    {
        for drive in drives {
            if let Err(err) = print_drive_info(drive) {
                println!("{err:?}");
            }
            println!();
        }
    } else {
        println!("no drives connected");
    }

    Ok(())
}

fn print_drive_info(drive: PathBuf) -> Result<()> {
    println!("Using drive {}", drive.display());
    let _drive = Drive::with_drive(drive)?;

    Ok(())
}
