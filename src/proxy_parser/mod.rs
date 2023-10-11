use std::io::Error;



pub mod clash_parser;


pub trait ProxyParser{
    fn port_from_path(path:String)->Result<String,Error>;
}