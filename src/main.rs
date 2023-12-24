use clap::Parser;
use std::path::PathBuf;
// get command line args that should be a path
// if that path exists,
// if that path is a directory, get all files in that directory
// optionally that search should be recursive
// optionally let the user provide file extension and only return those results

#[derive(Debug, Parser)]
pub struct CliArgs {
    path: PathBuf,

    #[arg(short = 'r')]
    recursive: bool,

    #[arg(long = "dry-run")]
    dry_run: bool,

    filetype: Option<String>,
}

mod file_search {
    use std::error::Error;
    use std::ffi::OsStr;
    use std::fs::{self};
    use std::path::PathBuf;
    pub fn walk_files(
        path: &PathBuf,
        recursive: bool,
        filetype: Option<String>,
    ) -> Result<Vec<PathBuf>, Box<dyn Error>> {
        if path.is_file() {
            return Ok(vec![path.clone()]);
        }
        let mut matches: Vec<PathBuf> = Vec::new();

        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            match path.is_dir() {
                true => {
                    // only search through sub-directories if recursive
                    if recursive == true {
                        matches.append(&mut walk_files(&path, recursive, filetype.clone()).unwrap())
                    }
                }
                // if filetype is defined, only add the specified type
                false => match &filetype {
                    None => matches.push(path),
                    Some(filetype) => {
                        let filetype = OsStr::new(&filetype);
                        if path.extension().unwrap() == filetype {
                            matches.push(path)
                        }
                    }
                },
            }
        }

        Ok(matches)
    }
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
