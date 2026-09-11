use std::fs::{create_dir_all, write};
use std::io::{Error, ErrorKind};
use std::path::Path;

pub fn make_file_if_not_exists(file_path: &Path) -> Result<(), Error> {
    // check if it is directory
    if file_path.is_dir() {
        return Err(Error::new(
            ErrorKind::IsADirectory,
            "Given path is a directory",
        ));
    }

    // if exists, return ok
    if file_path.exists() {
        return Ok(());
    }
    // create parent directories
    create_dir_all(
        file_path
            .parent()
            .expect("Path to parent must not be root or empty"),
    )
    .expect("Unable to create directory");
    // create file now
    write(file_path, "").expect("file to be written");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Read;
    use std::path::PathBuf;
    use tempdir::TempDir;

    // Helper to read file contents (expects UTF-8)
    fn read_file_contents(path: &PathBuf) -> String {
        let mut s = String::new();
        let mut f = fs::File::open(path).expect("open file");
        f.read_to_string(&mut s).expect("read to string");
        s
    }

    #[test]
    fn make_file_when_exists() {
        let tmp_dir = TempDir::new("test_file_exists_no_change").unwrap();
        let file_path = tmp_dir.path().join("file.txt");

        // create file with any content
        let initial_content = "existing content";
        fs::write(&file_path, initial_content).unwrap();
        assert!(file_path.exists(), "file should exist before run");
        make_file_if_not_exists(&file_path).unwrap();
        assert!(file_path.exists(), "file should still be present after run");

        let content = read_file_contents(&file_path);
        assert_eq!(content, initial_content, "file content should be unchanged");
    }

    #[test]
    fn make_file_when_not_exists() {
        let tmp_dir = TempDir::new("test_file_not_exists_created").unwrap();
        let file_path = tmp_dir.path().join("newfile.txt");

        assert!(!file_path.exists(), "file should not be present before run");
        make_file_if_not_exists(&file_path).unwrap();
        assert!(file_path.exists(), "file should be present after run");
        let content = read_file_contents(&file_path);
        assert_eq!(content, "", "file should be empty after run");
    }

    #[test]
    fn make_file_when_not_exists_with_parents() {
        let tmp_dir = TempDir::new("test_file_and_parents_not_exist").unwrap();
        let file_path = tmp_dir.path().join("a").join("b").join("file.txt");

        assert!(!file_path.exists(), "file should not be present before run");
        make_file_if_not_exists(&file_path).unwrap();
        assert!(file_path.exists(), "file should be present after run");

        let content = read_file_contents(&file_path);
        assert_eq!(content, "", "file should be empty after run");
    }

    #[test]
    fn make_file_when_directory_given_as_input() {
        let tmp_dir = TempDir::new("test_directory_instead_of_file_error").unwrap();
        let dir_path = tmp_dir.path().join("some_file");

        // Create directory with the file name
        fs::create_dir_all(&dir_path).unwrap();

        let result = make_file_if_not_exists(&dir_path);
        assert!(result.is_err(), "raise error when give path is directory");
    }
}
