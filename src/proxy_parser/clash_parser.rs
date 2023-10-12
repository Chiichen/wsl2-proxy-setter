use std::{
    fs,
    io::{Error, ErrorKind},
    path::Path,
};

use super::ProxyParser;
use yaml_rust::{self, YamlLoader};

pub struct ClashParser {}

impl ProxyParser for ClashParser {
    fn port_from_path(path_str: String) -> Result<String, Error> {
        let path = Path::new(&path_str);
        if !path.is_file() {
            println!("Can not find the file :{:?}", path_str);
            return Err(Error::from(ErrorKind::NotFound));
        }
        let extension = path.extension();
        if extension.is_none() {
            println!(
                "Error to get the extension of the file with the path :{:?}",
                path_str
            );
            return Err(Error::from(ErrorKind::NotFound));
        }
        let yaml_str = fs::read_to_string(path)?;

        let docs = YamlLoader::load_from_str(&yaml_str).unwrap();

        // // Multi document support, doc is a yaml::Yaml
        let doc = &docs[0];

        // // Debug support
        // println!("{:?}", doc);
        let port_str = doc["mixed-port"].as_i64();
        if port_str.is_none() {
            println!(" can not find port config in file :{:?}", path_str);
            return Err(Error::from(ErrorKind::NotFound));
        } else {
            return Ok(port_str.unwrap().to_string());
        }

        // // Index access for map & array
        // assert_eq!(doc["foo"][0].as_str().unwrap(), "list1");
        // assert_eq!(doc["bar"][1].as_f64().unwrap(), 2.0);

        // // Chained key/array access is checked and won't panic,
        // // return BadValue if they are not exist.
        // assert!(doc["INVALID_KEY"][100].is_badvalue());

        // // Dump the YAML object
        // let mut out_str = String::new();
        // {
        //     let mut emitter = YamlEmitter::new(&mut out_str);
        //     emitter.dump(doc).unwrap(); // dump the YAML object to a String
        // }
        // println!("{}", out_str);
    }
}
