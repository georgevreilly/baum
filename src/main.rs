use std::io;
use std::fs;

fn walk(root: &str, prefix: &str) -> io::Result<()> {
    for e in fs::read_dir(root)? {
        let path = e?.path();
        let name = path.file_name().unwrap().to_str().unwrap();
        println!("{}├── {}", prefix, name);
        if path.is_dir() {
            walk(&format!("{}/{}", root, name), &format!("{}├  ", prefix))?;
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let root = ".";
    walk(&root, "")?;
    Ok(())
}
