mod filetree;

use filetree::*;
use std::io;

fn print_tree(dir: Directory) {
    const OTHER_CHILD: &str = "│   ";
    const OTHER_ENTRY: &str = "├── ";
    const FINAL_CHILD: &str = "    ";
    const FINAL_ENTRY: &str = "└── ";

    fn visit(node: Directory, prefix: &str) {
        let mut count = node.entries.len();
        for entry in node.entries {
            count -= 1;
            let line = if count == 0 { FINAL_ENTRY } else { OTHER_ENTRY };
            match entry {
                FileTree::Dir(subdir) => {
                    println!("{}{}{}", prefix, line, subdir.name);
                    let child = if count == 0 { FINAL_CHILD } else { OTHER_CHILD };
                    visit(subdir, &format!("{}{}", prefix, child))
                }
                FileTree::Symlink(symlink) => {
                    println!("{}{}{} -> {}", prefix, line, symlink.name, symlink.target);
                }
                FileTree::File(file) => {
                    println!("{}{}{}", prefix, line, file.name);
                }
            }
        }
    }

    println!("{}", dir.name);
    visit(dir, "")
}

fn main() -> io::Result<()> {
    let root = ".";
    let dir = walk(&root.into())?;
    // println!("{:#?}", dir);
    print_tree(dir);
    Ok(())
}
