use std::path::Path;

use fs_extra::dir::{CopyOptions, copy};
use log::{debug, error, info};

use crate::{
    arguments::args::Args,
    command::run_command::run_command,
    config::config::{Config, ProfileInfo},
};

pub fn build_addon(args: Args) {
    let config = Config::get_config(&args.directory);

    let src_bp = Path::new(&args.directory).join("src/BP");
    let src_rp = Path::new(&args.directory).join("src/RP");
    let bp_dest = Path::new(&config.paths.bp_path).join(format!("{}_BP", config.addon_name));
    let rp_dest = Path::new(&config.paths.rp_path).join(format!("{}_RP", config.addon_name));

    let mut options = CopyOptions::new();
    options.overwrite = true;
    options.copy_inside = true;

    let profile: Option<&ProfileInfo> = match &args.profile {
        Some(name) => Some(config.get_profile(name).unwrap_or_else(|| {
            error!("Profile {} not found!", name);
            std::process::exit(1);
        })),
        None => None,
    };

    if let Some(before_build) = profile.and_then(|p| p.before_build.as_ref()) {
        for cmd in before_build {
            match run_command(&cmd.command) {
                Ok(status) if status.success() => {
                    debug!("Command succeeded: {}", cmd.command);
                }
                Ok(status) => {
                    error!("Command failed ({}): {}", status, cmd.command);
                }
                Err(e) => {
                    error!("Failed to run '{}': {}", cmd.command, e);
                }
            }
        }
    }

    info!("Copying files...");

    copy(&src_bp, &bp_dest, &options).unwrap();
    copy(&src_rp, &rp_dest, &options).unwrap();

    if let Some(before_build) = profile.and_then(|p| p.after_build.as_ref()) {
        for cmd in before_build {
            match run_command(&cmd.command) {
                Ok(status) if status.success() => {
                    debug!("Command succeeded: {}", cmd.command);
                }
                Ok(status) => {
                    error!("Command failed ({}): {}", status, cmd.command);
                }
                Err(e) => {
                    error!("Failed to run '{}': {}", cmd.command, e);
                }
            }
        }
    }

    info!("Done");
}
