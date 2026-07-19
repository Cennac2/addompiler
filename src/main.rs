use crate::{
    arguments::args::{ArgTypes, parse_args},
    build::build_addon,
    initialize::initialize_addompiler,
};

mod arguments;
mod build;
mod command;
mod config;
mod initialize;

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
            build_addon(args);
        }
    }
}
