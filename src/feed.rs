use std::path::Path;
use std::{fs, io};

pub fn clear(src: &str, enabled: bool) -> Result<(), io::Error> {
    if !enabled {
        return Ok(());
    }

    let path = Path::new(src);

    if path.exists() {
        fs::remove_file(path).expect("Failed to remove feed output");
        println!("Removed... {:#?}", path);
    }

    Ok(())
}
