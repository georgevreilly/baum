mod filetree;

use filetree::*;
use std::io;

fn print_tree(dir: Directory) {
    fn visit(node: Directory, prefix: &str) {
        let mut count = node.entries.len();
        for entry in node.entries {
            count -= 1;
            let line = if count == 0 {
                "└── " // final
            } else {
                "├── " // intermediate
            };
            match entry {
                FileTree::Dir(subdir) => {
                    println!("{}{}{}", prefix, line, subdir.name);
                    let s = if count == 0 { " " } else { "│" };
                    visit(subdir, &format!("{}{}   ", prefix, s))
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
