use std::io;

mod filetree;

/*
fn print_tree(ft: FileTree) {
    fn visit(node: FileTree, prefix: &str) {

        // println!("{}├── {}", prefix, name);
            visit(&format!("{}├  ", prefix))?,
    }
}
*/

fn main() -> io::Result<()> {
    let root = ".";
    let tree = filetree::walk(&root.into())?;
    println!("{:#?}", tree);
    Ok(())
}
