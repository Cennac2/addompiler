use std::path::Path;

use log::{debug, error, warn};

use crate::{
    arguments::args::Args,
    command::{copy::copy_ignoring, run_command::run_command},
    config::config::{CommandInfo, Config, ProfileInfo},
};

pub fn build_addon(args: &Args, config: Option<&Config>) {
    let config = if config.is_some() {
        config.unwrap()
    } else {
        &Config::get_config(&args.directory)
    };

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

    let profile: Option<&ProfileInfo> = match &args.profile {
        Some(name) => Some(config.get_profile(name).unwrap_or_else(|| {
            error!("Profile {} not found!", name);
            std::process::exit(1);
        })),
        None => None,
    };

    run_hooks(profile.and_then(|p| p.before_build.as_ref()));

    debug!("Copying files...");

    let ignored: Vec<String> = profile
        .and_then(|p| p.ignored_files.clone())
        .unwrap_or_default();

    match &bp_dest {
        Some(dest) => {
            copy_ignoring(&src_bp, dest, &ignored).unwrap();
        }
        None => warn!("bp_path not configured, skipping BP copy"),
    }

    match &rp_dest {
        Some(dest) => {
            copy_ignoring(&src_rp, dest, &ignored).unwrap();
        }
        None => warn!("rp_path not configured, skipping RP copy"),
    }
    run_hooks(profile.and_then(|p| p.after_build.as_ref()));

    debug!("Done");
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
