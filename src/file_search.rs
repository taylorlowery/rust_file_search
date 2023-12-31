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

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    pub fn path_to_test_files() -> PathBuf {
        PathBuf::from("./test_files")
    }

    #[rstest]
    #[case(true, None, 4)]
    #[case(false, None, 2)]
    #[case(false, Some(String::from("mp3")), 1)]
    #[case(true, Some(String::from("mp3")), 2)]
    #[case(true, Some(String::from("jpg")), 0)]
    fn test_walk_files(
        path_to_test_files: PathBuf,
        #[case] recursive: bool,
        #[case] file_type: Option<String>,
        #[case] expected: usize,
    ) {
        let results = walk_files(&path_to_test_files, recursive, file_type);
        assert_eq!(expected, results.unwrap().len());
    }

    #[rstest]
    #[should_panic]
    fn test_walk_files_invalid_path() {
        let results = walk_files(&PathBuf::from("INVALID_PATH"), false, None);
        assert_eq!(10, results.unwrap().len());
    }
}
