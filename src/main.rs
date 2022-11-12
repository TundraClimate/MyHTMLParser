use my_html_parser::config;
use my_html_parser::io::mhl_reader;

fn main() {
    let args = std::env::args();
    let config = config::load(args);
    let mhl_content = mhl_reader::read(&config.from_path);
}
