use clap::Parser;
use std::path::PathBuf;

// get command line args that should be a path
// if that path exists,
// if that path is a directory, get all files in that directory
// optionally that search should be recursive
// optionally let the user provide file extension and only return those results

#[derive(Debug, Parser)]
struct CliArgs {
    path: PathBuf,

    #[arg(short = 'r')]
    recursive: bool,

    #[arg(long = "dry-run")]
    dry_run: bool,

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
}
