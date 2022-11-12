use std::env::Args;
use std::process;
use std::path::PathBuf;

pub struct Config {
    pub from_path: PathBuf,
    pub out_path: PathBuf,
}

pub fn load(args: Args) -> Config {
    let conf = load_config(args);
    if let Err(e) = &conf {
        eprintln!("load error: {}", e);
        process::exit(1);
    }
    conf.unwrap()
}

fn load_config(args: Args) -> Result<Config, String> {
    let args: Vec<_> = args.collect();
    if check_from_path(&args) {} else { return Err(String::from("Not Specified *.mhl path.")); }
    let from_path = PathBuf::from(&args[1]);
    let mut out_path = PathBuf::from(String::from("."));
    if check_out_path(&args) { out_path = PathBuf::from(&args[2]) }
    Ok(Config { from_path, out_path })
}

fn check_from_path(args: &Vec<String>) -> bool {
    args.len() >= 2
}

fn check_out_path(args: &Vec<String>) -> bool {
    args.len() >= 3
}