use std::io;
use std::process::Command;
use std::path::{Path, PathBuf};

pub fn archive(src_dir: &Path, dest_dir: &Path) -> Result<PathBuf, io::Error> {
    let base_dir = path_to_str(src_dir.parent().expect("unable to get parent directory"));
    let item_name = src_dir.file_name()
        .expect("unable to get directory name")
        .to_str()
        .expect("non-unicode path");
    let tar_bz_filename = format!("{}.tar.bz2", item_name);
    let dest_file = dest_dir.join(tar_bz_filename);
    let output = Command::new("tar")
        .args(&["-cjf", &dest_file.to_str().expect("non-unicode path"), "-C", base_dir, item_name])
        .output()?;
    assert!(output.status.success());
    Ok(dest_file)
}

fn path_to_str(path: &Path) -> &str {
    path.to_str().expect("non-unicode path")
}


#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    extern crate tempdir;

    #[test]
    fn archive_is_created() -> Result<(), io::Error> {
        // set up the test directory
        let test_dir = tempdir::TempDir::new("archive_is_created")?;
        let src_dir = test_dir.path().join("src-dir");
        fs::create_dir(&src_dir)?;
        let test_content = "Archiving test";
        fs::write(src_dir.join("file.txt"), test_content)?;
        let dest_dir = test_dir.path().join("dest-dir");
        fs::create_dir(&dest_dir)?;

        let archive_file = archive(src_dir.as_path(), dest_dir.as_path())?;
        // check that archive can be unpacked and has the expected contents
        let unpack_dir = test_dir.path().join("unpack-dir");
        fs::create_dir(&unpack_dir)?;
        let output = Command::new("tar")
            .current_dir(&unpack_dir)
            .args(&["-xf", path_to_str(archive_file.as_path())])
            .output()?;
        assert!(output.status.success());
        let expected_unpacked_file = unpack_dir.join("src-dir").join("file.txt");
        assert!(expected_unpacked_file.is_file());
        let read_content = fs::read_to_string(expected_unpacked_file)?;
        assert_eq!(read_content, test_content);
        Ok(())
    }
}
