use jwalk::WalkDir;

use std::path::{Path, PathBuf};

fn is_cargo_toml(path: &Path) -> bool {
    if let Some(name) = path.file_name() {
        name == Path::new("Cargo.toml")
    } else {
        false
    }
}

fn find_cargo_tomls(path: impl AsRef<Path>) -> impl Iterator<Item = PathBuf> {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|path| is_cargo_toml(path))
}

pub fn list(path: impl AsRef<Path>) {
    for p in find_cargo_tomls(path) {
        println!("{}", p.display())
    }
}
