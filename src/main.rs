use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Debug)]
enum Entry {
    Directory((String, Vec<Entry>)),
    File(String), // TODO: metadata
    Symlink(String),
}

fn walk(root: &PathBuf, prefix: &str) -> io::Result<Entry> {
    let entries: Vec<fs::DirEntry> = fs::read_dir(root)?.map(|e| e.unwrap()).collect();
    let mut directory: Vec<Entry> = Vec::with_capacity(entries.len());
    for e in entries {
        let path = e.path();
        let name: String = path.file_name().unwrap().to_str().unwrap().into();
        if name.starts_with(".") {
            continue;
        };
        // println!("{}├── {}", prefix, name);
        let e2 = match path {
            path if path.is_dir() => walk(&root.join(name), &format!("{}├  ", prefix))?,
            path if path.is_symlink() => Entry::Symlink(name.into()),
            _ => Entry::File(name.into()),
        };
        directory.push(e2);
    }
    let name = root
        .file_name()
        .unwrap_or(OsStr::new("."))
        .to_str()
        .unwrap()
        .into();
    Ok(Entry::Directory((name, directory)))
}

fn main() -> io::Result<()> {
    let root = ".";
    let tree = walk(&root.into(), "")?;
    println!("{:#?}", tree);
    Ok(())
}
