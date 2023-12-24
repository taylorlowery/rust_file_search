use clap::Parser;
use std::path::PathBuf;

mod file_search;

#[derive(Debug, Parser)]
pub struct CliArgs {
    path: PathBuf,

    #[arg(short = 'r')]
    recursive: bool,

    filetype: Option<String>,
}

fn main() {
    let args = CliArgs::parse();
    println!("Hello, world!");
    println!("{:?}", args);
    match args.path.exists() {
        true => println!("Eureka!"),
        false => println!("Alas!"),
    }
    for path in file_search::walk_files(&args.path, args.recursive, args.filetype).unwrap() {
        println!("{:?}", path);
    }
}
