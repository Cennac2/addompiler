use glob::Pattern;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub fn copy_ignoring(src: &Path, dest: &Path, ignored: &[String]) -> std::io::Result<()> {
    let patterns: Vec<Pattern> = ignored
        .iter()
        .filter_map(|p| Pattern::new(p).ok())
        .collect();

    let is_ignored = |rel_path: &Path| {
        let rel_str = rel_path.to_string_lossy();
        patterns.iter().any(|pat| pat.matches(&rel_str))
            || rel_path
                .file_name()
                .map(|name| {
                    patterns
                        .iter()
                        .any(|pat| pat.matches(&name.to_string_lossy()))
                })
                .unwrap_or(false)
    };

    for entry in WalkDir::new(src).into_iter().filter_entry(|e| {
        let rel = e.path().strip_prefix(src).unwrap_or(e.path());
        rel.as_os_str().is_empty() || !is_ignored(rel)
    }) {
        let entry = entry.map_err(std::io::Error::other)?;
        let rel = entry.path().strip_prefix(src).unwrap();
        let target = dest.join(rel);

        if entry.file_type().is_dir() {
            fs::create_dir_all(&target)?;
        } else {
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(entry.path(), &target)?;
        }
    }

    Ok(())
}
