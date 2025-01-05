use std::path::Path;
use std::{fs::DirEntry, path::PathBuf};

pub fn ls<P: AsRef<Path>>(path: P) -> anyhow::Result<Vec<PathBuf>> {
    let mut entries = Vec::new();
    for entry in path.as_ref().read_dir()? {
        let entry = entry?;
        let entry_meta = entry.metadata()?;
        entries.push(entry.path());
        if entry_meta.is_dir() {
            entries.append(&mut ls(entry.path())?);
        }
    }
    Ok(entries)
}

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
    use std::fs::{File, create_dir_all, remove_dir_all};
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
    fn test_ls() -> anyhow::Result<()> {
        let temp_dir = std::env::temp_dir().join("durs_test_ls");
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
        let mut expected = vec![file_path, dir_path, file_path_from_dir];
        expected.sort();
        assert_eq!(actual, expected);

        Ok(())
    }
}
