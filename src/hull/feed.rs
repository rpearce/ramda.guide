use super::config::Config;
use std::path::Path;
use std::{fs, io};

pub fn remove(hull_opts: &Config) -> Result<(), io::Error> {
    let src = &hull_opts.feed.output;
    let path = Path::new(src);

    if path.exists() {
        fs::remove_file(path).expect("Hull: failed to remove feed output");
        println!("Hull: removed {:#?}", path);
    }

    Ok(())
}
