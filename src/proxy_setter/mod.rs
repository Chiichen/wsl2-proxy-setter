use std::io::Error;

pub mod UbuntuSetter;

pub trait ProxySetter {
    fn set_proxy(port: String) -> Result<String, Error>;
}
