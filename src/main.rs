mod filetree;

use filetree::*;
use std::env;
use std::io;
use std::path::PathBuf;

fn print_tree(root: &str, dir: Directory) {
    const OTHER_CHILD: &str = "│   ";
    const OTHER_ENTRY: &str = "├── ";
    const FINAL_CHILD: &str = "    ";
    const FINAL_ENTRY: &str = "└── ";

    fn visit(node: Directory, prefix: &str) -> (usize, usize) {
        let mut dirs: usize = 1; // counting this directory
        let mut files: usize = 0;
        let mut count = node.entries.len();
        for entry in node.entries {
            count -= 1;
            let line = if count == 0 { FINAL_ENTRY } else { OTHER_ENTRY };
            match entry {
                FileTree::Dir(subdir) => {
                    println!("{}{}{}", prefix, line, subdir.name);
                    let child = if count == 0 { FINAL_CHILD } else { OTHER_CHILD };
                    let (d, f) = visit(subdir, &format!("{}{}", prefix, child));
                    dirs += d;
                    files += f;
                }
                FileTree::Symlink(symlink) => {
                    println!("{}{}{} -> {}", prefix, line, symlink.name, symlink.target);
                    files += 1;
                }
                FileTree::File(file) => {
                    println!("{}{}{}", prefix, line, file.name);
                    files += 1;
                }
            }
        }
        (dirs, files)
    }

    println!("{}", root);
    let (d, f) = visit(dir, "");
    println!("\n{} directories, {} files", d, f)
}

fn main() -> io::Result<()> {
    let root = env::args().nth(1).unwrap_or(".".to_string());
    let dir: Directory = walk(&PathBuf::from(root.clone()), is_not_hidden, sort_by_name)?;
    print_tree(&root, dir);
    Ok(())
}
