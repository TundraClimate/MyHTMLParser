use std::fs;
use std::process;
use std::path::PathBuf;

pub fn read(path: &PathBuf) -> String {
    let mhl_file = read_mhl_file(path);
    if let Err(e) = mhl_file {
        eprintln!("read error: {}", e);
        process::exit(1);
    }
    mhl_file.unwrap()
}

fn read_mhl_file(path: &PathBuf) -> Result<String, String> {
    if path.extension().unwrap() != "mhl" { return Err(String::from("Different Extension")) }
    let file_content = fs::read_to_string(path);
    match file_content {
        Ok(file) => Ok(file),
        Err(e) => return Err(e.to_string()),
    }
}