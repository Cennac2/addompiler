use include_dir::{Dir, include_dir};
use std::path::Path;

static TEMPLATES: Dir = include_dir!("$CARGO_MANIFEST_DIR/addompile_templates");

pub fn extract_template(template_name: &str, dest: &Path) -> std::io::Result<()> {
    let template_dir = TEMPLATES
        .get_dir(template_name)
        .expect("template not found");

    extract_dir_recursive(template_dir, template_dir.path(), dest)
}

fn extract_dir_recursive(dir: &Dir, base: &Path, dest: &Path) -> std::io::Result<()> {
    for entry in dir.entries() {
        match entry {
            include_dir::DirEntry::Dir(subdir) => {
                extract_dir_recursive(subdir, base, dest)?;
            }
            include_dir::DirEntry::File(file) => {
                let relative = file.path().strip_prefix(base).unwrap();
                let out_path = dest.join(relative);

                if let Some(parent) = out_path.parent() {
                    std::fs::create_dir_all(parent)?;
                }
                std::fs::write(out_path, file.contents())?;
            }
        }
    }
    Ok(())
}
