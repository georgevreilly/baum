use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Directory {
    pub name: String,
    pub entries: Vec<FileTree>,
}

#[derive(Debug)]
pub struct File {
    pub name: String,
    pub metadata: fs::Metadata,
}

#[derive(Debug)]
pub struct Symlink {
    pub name: String,
    pub target: String,
    pub metadata: fs::Metadata,
}

#[derive(Debug)]
pub enum FileTree {
    Dir(Directory),
    File(File),
    Symlink(Symlink),
}

pub fn is_not_hidden(name: &str) -> bool {
    return !name.starts_with('.')
}

pub fn walk(root: &PathBuf, filter: fn(name: &str)-> bool) -> io::Result<Directory> {
    let entries: Vec<fs::DirEntry> = fs::read_dir(root)?.map(|e| e.unwrap()).collect();
    let mut directory: Vec<FileTree> = Vec::with_capacity(entries.len());
    for e in entries {
        let path = e.path();
        let name: String = path.file_name().unwrap().to_str().unwrap().into();
        if !filter(&name) {
            continue;
        };
        let metadata = fs::metadata(&path).unwrap();
        let entry = match path {
            path if path.is_dir() => FileTree::Dir(walk(&root.join(name), filter)?),
            path if path.is_symlink() => FileTree::Symlink(Symlink {
                name: name.into(),
                target: fs::read_link(path).unwrap().to_string_lossy().to_string(),
                metadata: metadata,
            }),
            path if path.is_file() => FileTree::File(File {
                name: name.into(),
                metadata: metadata,
            }),
            _ => unreachable!(),
        };
        directory.push(entry);
    }
    let name = root
        .file_name()
        .unwrap_or(OsStr::new("."))
        .to_str()
        .unwrap()
        .into();
    Ok(Directory {
        name: name,
        entries: directory,
    })
}
