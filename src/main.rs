use std::process;
use my_html_parser::config;
use my_html_parser::io::mhl_reader;

fn main() {
    let args = std::env::args();
    let config = config::load_config(args);
    if let Err(e) = &config {
        eprintln!("error: {}", e);
        process::exit(1);
    }
    let config = config.unwrap();
}
