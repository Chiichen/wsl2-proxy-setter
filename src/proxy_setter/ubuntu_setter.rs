use std::{
    fs,
    io::{Error, ErrorKind},
    path::Path,
    process::Command,
};

use super::ProxySetter;

pub struct UbuntuSetter;

impl ProxySetter for UbuntuSetter {
    // read ~/.bashrc file and edit the item http_proxy as "http://${hostip}:port"
    fn set_proxy(hostip: String, port: String) -> Result<(), std::io::Error> {
        let file_path = Path::new("~/.bashrc");
        if !file_path.exists() {
            println!("File ~/.bashrc does not exist.");
            return Err(Error::from(ErrorKind::NotFound));
        }
        let original_content = fs::read_to_string(file_path)?;
        let new_content = original_content
            .replace(
                r"^export\s+http_proxy(?=;|$)",
                &format!(" export http_proxy=http://{hostip}:{port}"),
            )
            .replace(
                r"^export\s+https_proxy(?=;|$)",
                &format!(" export https_proxy=https://{hostip}:{port}"),
            );
        fs::write(file_path, new_content)?;
        Ok(())
    }

    /// execute command cat /etc/resolv.conf |grep -oP '(?<=nameserver\ ).*' and return the output of this command
    /// if any output except of a valid ip address occured, return an error
    ///
    /// ## return value
    ///
    /// - `Ok(hostip)` return the hostip
    /// - `Err(Error)` return an error
    ///
    /// ## example
    ///
    /// ```rust
    /// use proxy_setter::setter::ubuntu_setter::UbuntuSetter;
    /// use proxy_setter::setter::ProxySetter;
    /// use std::io::Error;
    ///
    /// fn main() -> Result<(), Error> {
    ///     let hostip = UbuntuSetter::read_hostip()?;
    ///     println!("{}", hostip);
    ///     Ok(())
    /// ```
    /// }
    fn read_hostip() -> Result<String, Error> {
        let cmd = r"cat /etc/resolv.conf | grep -oP '(?<=nameserver\ ).*'";
        let output = Command::new("bash")
            .arg("-c")
            .arg(cmd)
            .output()
            .map_err(|_| {
                return Error::new(ErrorKind::Other, "fail to execute cat and grep command");
            })?;
        let raw_output = String::from_utf8(output.stdout)
            .map_err(|_| return Error::new(ErrorKind::Other, "fail to read the output"))?;
        let r = raw_output.split('\n').nth(0);
        if r.is_none() {
            println!("No valid ip address found in /etc/resolv.conf");
            return Err(Error::new(
                ErrorKind::Other,
                "No valid ip address found in /etc/resolv.conf",
            ));
        }
        let hostip = r.unwrap();
        if hostip.is_empty() {
            return Err(Error::new(
                ErrorKind::Other,
                "No valid ip address found in /etc/resolv.conf",
            ));
        }
        Ok(hostip.to_string())
    }
}
