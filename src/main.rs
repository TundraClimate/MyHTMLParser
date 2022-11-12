use my_html_parser::config;
use my_html_parser::io::mhl_reader;

fn main() {
    let args = std::env::args();
    let config = config::load_config(&args);
}
