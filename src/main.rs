use crate::{
    arguments::args::{ArgTypes, parse_args},
    build::build_addon,
    initialize::initialize_addompiler,
};

mod arguments;
mod build;
mod config;
mod initialize;

fn main() {
    let args = parse_args();

    env_logger::Builder::new()
        .filter_level(if args.debug {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Info
        })
        .init();

    match args.arg_type {
        ArgTypes::Init => {
            initialize_addompiler(args);
        }
        ArgTypes::Build => {
            build_addon(args);
        }
    }
}
