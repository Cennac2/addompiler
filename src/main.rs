use log::error;

use crate::{
    arguments::args::{ArgTypes, parse_args},
    build::build_addon,
    initialize::initialize_addompiler,
    watch::watch_addon,
};

mod arguments;
mod build;
mod command;
mod config;
mod initialize;
mod watch;

fn main() {
    let args = parse_args();

    use std::io::Write;

    env_logger::Builder::new()
        .filter_level(if args.debug {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Info
        })
        .format(|buf, record| {
            let level_style = buf.default_level_style(record.level());
            writeln!(
                buf,
                "[{level_style}{}{level_style:#}] {}",
                record.level(),
                record.args()
            )
        })
        .init();

    match args.arg_type {
        ArgTypes::Init => {
            initialize_addompiler(args);
        }
        ArgTypes::Build => {
            build_addon(&args, None);
        }
        ArgTypes::Watch => {
            if let Err(e) = watch_addon(args) {
                error!("Failed to start watcher: {}", e);
                std::process::exit(1);
            }
        }
    }
}
