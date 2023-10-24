use proxy_parser::{clash_parser::ClashParser, ProxyParser};
pub mod proxy_parser;
pub mod proxy_setter;

fn main() {
    // println!(
    //     "ans:{:?}",
    //     ClashParser::port_from_path("/mnt/c/Users/Chi/.config/clash/config.yaml".to_string())
    // );
    println!(
        "ans:{:?}",
        ClashParser::port_from_path("C:\\Users\\Chi\\.config\\clash\\config.yaml".to_string())
    );
}
