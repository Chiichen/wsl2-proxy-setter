use std::io::Error;

pub mod ubuntu_setter;

pub trait ProxySetter {
    fn set_proxy(hostip: String, port: String) -> Result<(), Error>;
    fn read_hostip() -> Result<String, Error>;
}
