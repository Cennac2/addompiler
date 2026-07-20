use std::path::Path;

use log::{debug, error};

use crate::{
    arguments::args::Args, config::config::Config, templates_manager::templates::extract_template,
};

pub fn initialize_addompiler(args: Args) {
    Config::init_config(&args.directory);

    let dir = Path::new(&args.directory);
    let template = args.template;

    match extract_template(&template, dir) {
        Ok(_) => {
            debug!("Creating using template {}", template);
        }
        Err(e) => {
            error!("Failed to create template {}: {}", template, e);
        }
    };
}
