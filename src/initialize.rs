use std::{fs, path::Path};

use log::{debug, error};

use crate::{arguments::args::Args, config::config::Config};

pub fn initialize_addompiler(args: Args) {
    Config::init_config(&args.directory);

    let dir = Path::new(&args.directory);

    for sub in ["src", "src/RP", "src/BP"] {
        match fs::create_dir(dir.join(sub)) {
            Ok(_) => {
                debug!("Creating {}", sub);
            }
            Err(e) => {
                error!("Failed to create {}: {}", sub, e);
            }
        }
    }
}
