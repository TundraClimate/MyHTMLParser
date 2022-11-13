use my_html_parser::config;
use my_html_parser::io::mhl_reader;
use my_html_parser::parse::parser;
use my_html_parser::io::html_writer;

fn main() {
    let args = std::env::args();
    let config = config::load(args);
    let mhl_content = mhl_reader::read(&config.from_path);
    let parsed_content = parser::parse_for_content(&mhl_content);
    html_writer::write(&config.out_path, &parsed_content);
}
