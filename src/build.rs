use std::path::Path;

use fs_extra::dir::{CopyOptions, copy};
use log::{debug, error, info, warn};

use crate::{
    arguments::args::Args,
    command::run_command::run_command,
    config::config::{CommandInfo, Config, ProfileInfo},
};

pub fn build_addon(args: Args) {
    let config = Config::get_config(&args.directory);

    let src_bp = Path::new(&args.directory).join("src/BP");
    let src_rp = Path::new(&args.directory).join("src/RP");

    let bp_dest = config
        .paths
        .bp_path
        .as_ref()
        .map(|p| Path::new(p).join(format!("{}_BP", config.addon_name)));
    let rp_dest = config
        .paths
        .rp_path
        .as_ref()
        .map(|p| Path::new(p).join(format!("{}_RP", config.addon_name)));

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

    run_hooks(profile.and_then(|p| p.before_build.as_ref()));

    info!("Copying files...");

    match &bp_dest {
        Some(dest) => {
            copy(&src_bp, dest, &options).unwrap();
        }
        None => warn!("bp_path not configured, skipping BP copy"),
    }

    match &rp_dest {
        Some(dest) => {
            copy(&src_rp, dest, &options).unwrap();
        }
        None => warn!("rp_path not configured, skipping RP copy"),
    }

    run_hooks(profile.and_then(|p| p.after_build.as_ref()));

    info!("Done");
}

fn run_hooks(commands: Option<&Vec<CommandInfo>>) {
    let Some(commands) = commands else { return };

    for cmd in commands {
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
