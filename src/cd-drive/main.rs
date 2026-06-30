use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use libcdio_rs::{
    Drive, Mmc,
    mmc::{MmcFeature, MmcProfile},
};

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
            print_mmc_features(&mmc);
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

fn print_mmc_features(mmc: &Mmc) {
    println!("{L1} Supported features:");
    let features = match mmc.features() {
        Ok(features) => features,
        Err(err) => {
            println!("{:?}", err);
            return;
        }
    };

    features.into_iter().for_each(|feature| {
        match feature {
            MmcFeature::ProfileList { ref profiles } => {
                println!("{L2} {} {}: ", to_char(true), feature);
                profiles.iter().for_each(|MmcProfile { active, kind }| {
                    println!("{L3} {} {}", to_char(*active), kind);
                })
            }
            MmcFeature::Core { interface } => {
                println!("{L2} {} {}:", to_char(true), feature);
                println!("{L3} {} Interface: {}", to_char1(true), interface);
            }
            MmcFeature::Morphing {
                async_events,
                op_chg_events,
            } => {
                println!("{L2} {} {}:", to_char(true), feature);
                println!("{L3} {} Asynchronous events", to_char1(async_events));
                println!("{L3} {} Operational change events", to_char1(op_chg_events));
            }
            MmcFeature::RemovableMedium {
                eject,
                lock,
                prevent_jumper,
                load_mech,
            } => {
                println!("{L2} {} {}:", to_char(true), feature);
                println!("{L3} {} Eject", to_char1(eject));
                println!("{L3} {} Lock", to_char1(lock));
                println!("{L3} {} Prevent Jumper", to_char1(prevent_jumper));
                println!("{L3} {} Loading Mechanism: {}", to_char1(true), load_mech);
            }
            MmcFeature::CdRead {
                active,
                c2_error,
                cd_text,
                dap,
            } => {
                println!("{L2} {} {}:", to_char(active), feature);
                println!("{L3} {} C2 Errors", to_char1(c2_error));
                println!("{L3} {} CD-Text", to_char1(cd_text));
                println!("{L3} {} DAP", to_char1(dap));
            }
            MmcFeature::CdAudioExternalPlay {
                active,
                scan,
                sep_channel_mute,
                sep_volume,
                volume_levels,
            } => {
                println!("{L2} {} {}:", to_char(active), feature);
                println!("{L3} {} SCAN", to_char1(scan));
                println!("{L3} {} Separate Channel Mute", to_char1(sep_channel_mute));
                println!("{L3} {} Separate Volume", to_char1(sep_volume));
                println!("{L3} {} Volume Levels: {}", to_char1(true), volume_levels);
            }
            MmcFeature::DvdCss { active, .. } => {
                println!("{L2} {} {}:", to_char(active), feature);
            }
            MmcFeature::DriveSerialNumber { ref sno } => {
                println!("{L2} {} {}:", to_char(true), feature);
                println!("{L3} {} S/N: {}", to_char1(true), sno);
            }
            _ => {
                println!("{L2} (?) {}:", feature);
            }
        }

        fn to_char(active: bool) -> &'static str {
            if active { "[*]" } else { "[ ]" }
        }
    });
}

const L1: &str = "  ";
const L2: &str = "     ";
const L3: &str = "        ";
