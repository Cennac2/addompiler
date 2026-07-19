use log::{error, info};
use notify_debouncer_mini::{new_debouncer, notify::RecursiveMode};
use std::path::Path;
use std::sync::mpsc;
use std::time::Duration;

use crate::arguments::args::Args;
use crate::build::build_addon;
use crate::config::config::Config;

pub fn watch_addon(args: Args) -> notify::Result<()> {
    let (tx, rx) = mpsc::channel();

    let mut debouncer = new_debouncer(Duration::from_millis(300), tx)?;

    let config = Config::get_config(&args.directory);

    debouncer
        .watcher()
        .watch(Path::new(&args.directory), RecursiveMode::Recursive)?;

    info!("Watching {} for changes...", args.directory);

    build_addon(&args, Some(&config));

    for result in rx {
        match result {
            Ok(events) if !events.is_empty() => {
                info!("Detected {} change(s), rebuilding...", events.len());
                build_addon(&args, Some(&config));
            }
            Ok(_) => {}
            Err(e) => error!("Watch error: {:?}", e),
        }
    }

    Ok(())
}
