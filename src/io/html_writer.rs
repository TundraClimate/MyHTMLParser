use std::ffi::OsStr;
use std::process;
use std::path::PathBuf;
use std::fs;

pub fn write(path: &PathBuf, content: &String) {
    let write = write_html_file(path, content);
    if let Err(e) = write {
        eprintln!("write error: {}", e);
        process::exit(1);
    }
}

fn write_html_file(path: &PathBuf, content: &String) -> Result<(), String> {
    check_extension(path, path.extension())?;
    if path.exists() {
        eprintln!("{} file overwrite.", path.display());
        if let Err(_) = fs::remove_file(path) {
            eprintln!("FIle Overwrite Failed.")
        }
    }
    let write = fs::write(path, content);
    match write {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string())
    }
}

fn check_extension(path: &PathBuf, extension: Option<&OsStr>) -> Result<(), String> {
    match extension {
        None => return Err(format!("Invalid Path. {} is Directory", path.display())),
        _ => {
            if extension.unwrap() != "html" {
                return Err(String::from("Different Extension"))
            }
        }
    }
    Ok(())
}