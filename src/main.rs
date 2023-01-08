mod filetree;

use std::io;
use filetree::*;


fn print_tree(dir: Directory) {
    fn visit(node: Directory, prefix: &str) {
        println!("{} {}", prefix, node.name);
        for entry in node.entries {
            match entry {
                FileTree::Dir(subdir) => {
                    // println!("{}├── {}", prefix, subdir.name);
                    visit(subdir, &format!("{}├  ", prefix))
                },
                FileTree::Symlink(symlink) => {
                    println!("{}├── {} -> {}", prefix, symlink.name, symlink.target);
                },
                FileTree::File(file) => {
                    println!("{}├── {}", prefix, file.name);
                },
            }
        }
    }

    visit(dir, "")
}

fn main() -> io::Result<()> {
    let root = ".";
    let dir = walk(&root.into())?;
    // println!("{:#?}", dir);
    print_tree(dir);
    Ok(())
}
