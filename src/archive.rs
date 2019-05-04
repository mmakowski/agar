use std::process::Command;
use std::path::Path;

pub fn archive<'a>(src_dir: &Path, dest_dir: &'a Path) -> Result<&'a Path, ArchiveError> {
    let base_dir = src_dir.parent()
        .expect("unable to get parent directory")
        .to_str()
        .expect("TODO");
    let item_name = src_dir.file_name()
        .expect("unable to get directory name")
        .to_str()
        .expect("non-unicode directory name");
    let tar_bz_filename = format!("{}.tar.bz2", item_name);
    let output = Command::new("tar")
        .args(&["-cjf", &tar_bz_filename, "-C", &base_dir, item_name])
        .output()
        .expect("failed to execute tar");
    assert!(output.status.success());
    return Err(ArchiveError::Todo);
}

#[derive(Debug)]
pub enum ArchiveError {
    Todo
}

#[cfg(test)]
mod tests {
    #[test]
    fn archive_is_created() {
        // TODO: test that archive is created
        assert_eq!(2 + 2, 4);
    }
}
