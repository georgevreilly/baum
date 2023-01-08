use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Debug)]
struct DirEntry {
    name: String,
    entries: Vec<FileTree>,
}

#[derive(Debug)]
struct FileEntry {
    name: String,
    metadata: fs::Metadata,
}

#[derive(Debug)]
struct SymlinkEntry {
    name: String,
    target: String,
    metadata: fs::Metadata,
}

#[derive(Debug)]
enum FileTree {
    Directory(DirEntry),
    File(FileEntry),
    Symlink(SymlinkEntry),
}

fn walk(root: &PathBuf, prefix: &str) -> io::Result<FileTree> {
    let entries: Vec<fs::DirEntry> = fs::read_dir(root)?.map(|e| e.unwrap()).collect();
    let mut directory: Vec<FileTree> = Vec::with_capacity(entries.len());
    for e in entries {
        let path = e.path();
        let name: String = path.file_name().unwrap().to_str().unwrap().into();
        if name.starts_with(".") {
            continue;
        };
        // println!("{}├── {}", prefix, name);
        let metadata = fs::metadata(&path).unwrap();
        let fte = match path {
            path if path.is_dir() => walk(&root.join(name), &format!("{}├  ", prefix))?,
            path if path.is_symlink() => FileTree::Symlink(SymlinkEntry {
                name: name.into(),
                target: fs::read_link(path).unwrap().to_string_lossy().to_string(),
                metadata: metadata,
            }),
            path if path.is_file() => FileTree::File(FileEntry {
                name: name.into(),
                metadata: metadata,
            }),
            _ => unreachable!(),
        };
        directory.push(fte);
    }
    let name = root
        .file_name()
        .unwrap_or(OsStr::new("."))
        .to_str()
        .unwrap()
        .into();
    Ok(FileTree::Directory(DirEntry {
        name: name,
        entries: directory,
    }))
}

fn main() -> io::Result<()> {
    let root = ".";
    let tree = walk(&root.into(), "")?;
    println!("{:#?}", tree);
    Ok(())
}
