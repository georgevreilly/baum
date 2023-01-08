use std::io;

mod filetree;

fn main() -> io::Result<()> {
    let root = ".";
    let tree = filetree::walk(&root.into(), "")?;
    println!("{:#?}", tree);
    Ok(())
}
