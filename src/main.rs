use std::io;
use std::fs;
use std::path::PathBuf;

use walkdir::{DirEntry, WalkDir};

fn is_not_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| entry.depth() == 0 || !s.starts_with("."))
        .unwrap_or(false)
}

fn walkdir_tree(root: &PathBuf) {
    WalkDir::new(root)
        .into_iter()
        .filter_entry(|e| is_not_hidden(e))
        .filter_map(|v| v.ok())
        .for_each(|x| {
            let indent = if x.depth() > 0 {
                "│   ".repeat(x.depth() - 1)
            } else {
                "".to_owned()
            };
            println!("{}├── {}", indent, x.path().display())
        });
}

fn walk(root: &PathBuf) -> io::Result<()> {
    for e in fs::read_dir(root)? {
        let e = e?;
        let path = e.path();
        println!("{:?}", e);
        if path.is_dir() {
            walk(&path)?;
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let root = PathBuf::from(".");
    walk(&root)?;
    walkdir_tree(&root);
    Ok(())
}
