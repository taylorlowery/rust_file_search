# File Search

A simple little application that returns a list of files at a path. If the path is to a directory, returns all the files in that directory. Optionally will search recursively through any sub-directories. User also has the option to provide a file extension as a string(eh, `"mp3"`), and file search will return only the files whose extention matches that file type. 

## Usage

### CLI

`src/main.rs` is a little CLI application that will search for files matching commands and print out the list of results.

Recursively search a directory for all files:

`cargo run -- ./test_files/ -r`

Recursively search a directory for files with the extension "mp3":

`cargo run -- ./test_files/ -r mp3`

Search a particular directory, non-recursively, for txt files:

`cargo run -- ./test_files/ txt`

Search for a particular file. If it exists, cli will print just the path to this file:

`cargo run -- ./test_files/file_01.txt`

### Module

The CLI wraps the functionality of a module at `src/file_search.rs`: 


Recursively search a directory for files with the extension "mp3":

```rust

mod file_search;

let results = file_search::walk_files(&PathBuf::from("./test_files", true, Some(String::from("mp3"))))

``` 
