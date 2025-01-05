use std::path::Path;
use std::{fs::DirEntry, path::PathBuf};

/// Lists the contents of the specified path.
///
/// If the path is a directory, this function returns a vector of `PathBuf` objects representing
/// the files and directories within the directory.
/// If the path is a file, this function returns a vector containing the path of the file.
///
/// # Examples
///
/// ```
/// use durs::ls;
/// use std::path::Path;
///
/// let files = ls(Path::new("/path/to/directory"))?;
/// for file in files {
///     println!("{}", file.display());
/// }
/// ```
///
/// # Errors
///
/// This function will return an error if the path cannot be accessed or if there is an error
/// reading the directory contents.
pub fn ls<P: AsRef<Path>>(path: P) -> anyhow::Result<Vec<PathBuf>> {
    let meta = path.as_ref().symlink_metadata()?;
    let mut entries = Vec::new();
    if meta.is_dir() {
        for entry in path.as_ref().read_dir()? {
            entries.push(entry?.path());
        }
    } else {
        entries.push(path.as_ref().to_path_buf());
    }
    Ok(entries)
}

/// Recursively lists the contents of the specified path and all its subdirectories.
///
/// This function returns a vector of `PathBuf` objects representing all the files and directories
/// within the specified path and its subdirectories.
///
/// # Examples
///
/// ```
/// use durs::ls_rec;
/// use std::path::Path;
///
/// let files = ls_rec(Path::new("/path/to/directory"))?;
/// for file in files {
///     println!("{}", file.display());
/// }
/// ```
///
/// # Errors
///
/// This function will return an error if the path cannot be accessed or if there is an error
/// reading the directory contents.
pub fn ls_rec<P: AsRef<Path>>(path: P) -> anyhow::Result<Vec<PathBuf>> {
    let meta = path.as_ref().symlink_metadata()?;
    let mut entries = Vec::new();
    if meta.is_dir() {
        for entry in path.as_ref().read_dir()? {
            let entry = entry?;
            let entry_meta = entry.metadata()?;
            entries.push(entry.path());
            if entry_meta.is_dir() {
                entries.append(&mut ls_rec(entry.path())?);
            }
        }
    } else {
        entries.push(path.as_ref().to_path_buf());
    }
    Ok(entries)
}

/// Calculates the total size of the specified path and its contents.
///
/// If the path is a directory, this function recursively calculates the total size of all files
/// and subdirectories within the directory.
/// If the path is a file, this function returns the size of the file.
///
/// # Examples
///
/// ```
/// use durs::size;
/// use std::path::Path;
///
/// let total_size = size(Path::new("/path/to/file_or_directory"))?;
/// println!("Total size: {} bytes", total_size);
/// ```
///
/// # Errors
///
/// This function will return an error if the path cannot be accessed or if there is an error
/// reading the directory contents.
pub fn size<P: AsRef<Path>>(path: P) -> anyhow::Result<u64> {
    let meta = path.as_ref().symlink_metadata()?;
    let mut bytes = 0;
    if meta.is_dir() {
        for entry in path.as_ref().read_dir()? {
            let entry = entry?;
            let entry_meta = entry.metadata()?;
            if entry_meta.is_dir() {
                bytes += size(entry.path())?;
            } else {
                bytes += entry_meta.len();
            }
        }
    } else {
        bytes = meta.len();
    }
    Ok(bytes)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::{create_dir_all, remove_dir_all, File};
    use std::io::Write;
    use std::path::Path;

    #[test]
    fn test_size() -> anyhow::Result<()> {
        let temp_dir = std::env::temp_dir().join("durs_test_size");
        if temp_dir.exists() {
            remove_dir_all(&temp_dir)?;
        }
        create_dir_all(&temp_dir)?;

        let mut file = File::create(temp_dir.join("file"))?;
        write!(file, "test"); // 4 bytes

        let dir_path = temp_dir.join("dir");
        create_dir_all(&dir_path)?;

        let mut file = File::create(&dir_path.join("other_file"))?;
        write!(file, "testing test"); // 12 bytes

        let mut file = File::create(&dir_path.join("and_another_file"))?;
        write!(file, "testing test of tests"); // 21 bytes

        assert_eq!(size(&temp_dir)?, 4 + (12 + 21));

        Ok(())
    }

    #[test]
    fn test_ls_dir() -> anyhow::Result<()> {
        let temp_dir = std::env::temp_dir().join("durs_test_ls_dir");
        if temp_dir.exists() {
            remove_dir_all(&temp_dir)?;
        }
        create_dir_all(&temp_dir)?;

        let file_path = temp_dir.join("file");
        let _ = File::create(&file_path)?;

        let dir_path = temp_dir.join("dir");
        create_dir_all(&dir_path)?;
        let file_path_from_dir = dir_path.join("file");
        let _ = File::create(&file_path_from_dir)?;

        let mut actual = ls(&temp_dir)?;
        actual.sort();
        let mut expected = vec![file_path, dir_path];
        expected.sort();
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_ls_file() -> anyhow::Result<()> {
        let temp_dir = std::env::temp_dir().join("durs_test_ls_file");
        if temp_dir.exists() {
            remove_dir_all(&temp_dir)?;
        }
        create_dir_all(&temp_dir)?;

        let file_path = temp_dir.join("file");
        let _ = File::create(&file_path)?;

        let mut actual = ls(&file_path)?;
        let mut expected = vec![file_path];
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_ls_rec_dir() -> anyhow::Result<()> {
        let temp_dir = std::env::temp_dir().join("durs_test_ls_rec_dir");
        if temp_dir.exists() {
            remove_dir_all(&temp_dir)?;
        }
        create_dir_all(&temp_dir)?;

        let file_path = temp_dir.join("file");
        let _ = File::create(&file_path)?;

        let dir_path = temp_dir.join("dir");
        create_dir_all(&dir_path)?;
        let file_path_from_dir = dir_path.join("file");
        let _ = File::create(&file_path_from_dir)?;

        let mut actual = ls_rec(&temp_dir)?;
        actual.sort();
        let mut expected = vec![file_path, dir_path, file_path_from_dir];
        expected.sort();
        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn test_ls_rec_file() -> anyhow::Result<()> {
        let temp_dir = std::env::temp_dir().join("durs_test_ls_rec_file");
        if temp_dir.exists() {
            remove_dir_all(&temp_dir)?;
        }
        create_dir_all(&temp_dir)?;

        let file_path = temp_dir.join("file");
        let _ = File::create(&file_path)?;

        let mut actual = ls_rec(&file_path)?;
        let mut expected = vec![file_path];
        assert_eq!(actual, expected);

        Ok(())
    }
}
