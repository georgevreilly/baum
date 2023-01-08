mod filetree;

use filetree::*;
use std::io;

fn print_tree(dir: Directory) {
    fn visit(node: Directory, level: usize) {
        let prefix = "│  ".repeat(level);
        let mut count = node.entries.len();
        println!("{} {}", prefix, node.name);
        for entry in node.entries {
            count -= 1;
            let corner = if count == 0 { "└" } else { "├" };
            match entry {
                FileTree::Dir(subdir) => {
                    // println!("{}{}── {}", prefix, corner, subdir.name);
                    visit(subdir, level + 1)
                }
                FileTree::Symlink(symlink) => {
                    println!(
                        "{}{}── {} -> {}",
                        prefix, corner, symlink.name, symlink.target
                    );
                }
                FileTree::File(file) => {
                    println!("{}{}── {}", prefix, corner, file.name);
                }
            }
        }
    }

    visit(dir, 0)
}

fn main() -> io::Result<()> {
    let root = ".";
    let dir = walk(&root.into())?;
    // println!("{:#?}", dir);
    print_tree(dir);
    Ok(())
}
