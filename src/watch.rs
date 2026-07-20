use log::{debug, error, info};
use notify_debouncer_mini::{new_debouncer, notify::RecursiveMode};
use std::path::Path;
use std::sync::mpsc;
use std::time::Duration;

use crate::arguments::args::Args;
use crate::build::build_addon;
use crate::config::config::Config;

pub fn watch_addon(args: Args) -> notify::Result<()> {
    let (tx, rx) = mpsc::channel();

    let mut debouncer = new_debouncer(Duration::from_secs(1), tx)?;

    let config = Config::get_config(&args.directory);

    let profile = config.get_profile(&args.profile).unwrap_or_else(|| {
        error!("Profile {} not found!", &args.profile);
        std::process::exit(1);
    });

    let ignored: Vec<String> = profile.ignored_watch_files.clone().unwrap_or_default();

    debouncer
        .watcher()
        .watch(Path::new(&args.directory), RecursiveMode::Recursive)?;

    info!("Watching {} for changes...", args.directory);

    build_addon(&args, Some(&config));

    for result in rx {
        match result {
            Ok(events) if !events.is_empty() => {
                let relevant: Vec<_> = events
                    .iter()
                    .filter(|e| !is_ignored(&e.path, &ignored))
                    .collect();

                if !relevant.is_empty() {
                    info!("Detected {} change(s), rebuilding...", relevant.len());
                    build_addon(&args, Some(&config));
                } else {
                    debug!("Ignored {} change(s), skipping rebuild", events.len());
                }
            }
            Ok(_) => {}
            Err(e) => error!("Watch error: {:?}", e),
        }
    }

    Ok(())
}

fn is_ignored(path: &Path, ignored: &[String]) -> bool {
    let path_str = path.to_string_lossy();
    ignored
        .iter()
        .any(|pattern| path_str.contains(pattern.as_str()))
}
