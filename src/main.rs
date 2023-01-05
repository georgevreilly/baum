use std::io;
use std::fs;

#[derive(Debug)]
enum Entry {
    Directory((String, Vec<Entry>)),
    File(String), // TODO: metadata
    Symlink(String),
}

fn walk(root: &str, prefix: &str) -> io::Result<Entry> {
    let entries: Vec<fs::DirEntry> = fs::read_dir(root)?.map(|e| e.unwrap()).collect();
    let mut directory: Vec<Entry> = Vec::with_capacity(entries.len());
    for e in entries {
        let path = e.path();
        let name = path.file_name().unwrap().to_str().unwrap();
        // println!("{}├── {}", prefix, name);
        if path.is_dir() {
            let sub = walk(&format!("{}/{}", root, name), &format!("{}├  ", prefix))?;
            directory.push(sub);
        } else if path.is_symlink() {
            directory.push(Entry::Symlink(name.into()));
        } else {
            directory.push(Entry::File(name.into()));
        }
    }
    Ok(Entry::Directory((root.into(), directory)))
}

fn main() -> io::Result<()> {
    let root = ".";
    let tree = walk(&root, "")?;
    println!("{:?}", tree);
    Ok(())
}
