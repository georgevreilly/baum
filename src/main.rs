use walkdir::{DirEntry, WalkDir};

fn is_not_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| entry.depth() == 0 || !s.starts_with("."))
        .unwrap_or(false)
}

fn main() {
    let root = ".";
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
