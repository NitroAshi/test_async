use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
pub struct Args {
    path: Vec<PathBuf>,
}

pub fn input() -> Vec<PathBuf> {
    let args = Args::parse();
    return args.path;
}
