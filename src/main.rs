use std::env;
use std::io;
use std::path::PathBuf;

mod filetree;
mod treeprint;

use filetree::*;
use treeprint::*;

fn main() -> io::Result<()> {
    let root = env::args().nth(1).unwrap_or(".".to_string());
    let dir: Directory = dir_walk(&PathBuf::from(root.clone()), is_not_hidden, sort_by_name)?;
    print_tree(&root, dir);
    Ok(())
}
