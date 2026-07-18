use std::path::Path;

use fs_extra::dir::{CopyOptions, copy};
use log::info;

use crate::{arguments::args::Args, config::config::Config};

pub fn build_addon(args: Args) {
    let config = Config::get_config(&args.directory);

    let src_bp = Path::new(&args.directory).join("src/BP");
    let src_rp = Path::new(&args.directory).join("src/RP");
    let bp_dest = Path::new(&config.paths.bp_path).join(format!("{}_BP", config.addon_name));
    let rp_dest = Path::new(&config.paths.rp_path).join(format!("{}_RP", config.addon_name));

    let mut options = CopyOptions::new();
    options.overwrite = true;
    options.copy_inside = true;

    info!("Copying files...");

    copy(&src_bp, &bp_dest, &options).unwrap();
    copy(&src_rp, &rp_dest, &options).unwrap();

    info!("Done");
}
