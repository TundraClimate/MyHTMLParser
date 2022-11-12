use std::env::Args;
use std::path::PathBuf;

pub struct Config {
    pub from_path: PathBuf,
}

pub fn load_config(args: &Args) -> Config {
    let args: Vec<_> = args.collect();
    let from_path = PathBuf::from(String::from(&args[1]));
    Config { from_path }
}