use std::{
    fs,
    io::{Error, ErrorKind},
    path::Path,
};

use super::ProxySetter;

pub struct UbuntuSetter;

impl ProxySetter for UbuntuSetter {
    fn set_proxy(port: String) -> Result<String, std::io::Error> {
        let file_path = Path::new("~/.bashrc");
        if !file_path.exists() {
            println!("File ~/.bashrc does not exist.");
            return Err(Error::from(ErrorKind::NotFound));
        }

        let original_content = fs::read_to_string(file_path)?;

        for line in original_content.lines() {}
        return Ok(port);
    }
}
