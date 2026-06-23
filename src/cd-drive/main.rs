use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use libcdio_rs::{Drive, Mmc};

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

fn print_drive_info(path: PathBuf) -> Result<()> {
    println!("Using drive {}", path.display());
    let drive = Drive::with_drive(path.clone())?;

    if let Err(err) = print_device_info(&drive) {
        println!("{err:?}");
    };

    println!("MMC information:");
    match Mmc::with_device(path) {
        Err(err) => println!("{err:?}"),
        Ok(mmc) => {
            print_mmc_level(&mmc);
        }
    };

    Ok(())
}

fn print_device_info(drive: &Drive) -> Result<()> {
    println!("Device information:");
    let info = drive.hardware_info()?;
    println!("{L1} Vendor   : {}", info.vendor);
    println!("{L1} Model    : {}", info.model);
    println!("{L1} Revision : {}", info.revision);

    Ok(())
}

fn print_mmc_level(mmc: &Mmc) {
    let level = match mmc.level() {
        Ok(level) => level,
        Err(err) => {
            println!("{:?}", err);
            return;
        }
    };

    println!("{L1} Level    : {}", level);
}
