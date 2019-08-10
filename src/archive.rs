//! Packing and unpacking source directories as encrypted archives.
//!
//! At present it depends on system utilities: `tar` and `gpg`.
use std::fs;
use std::io;
use std::process::Command;
use std::path::{Path, PathBuf};

const CIPHER: &str = "AES256";

/// Packs and encrypts the specified directory. Returns the path to the archive file created.
///
/// # Arguments
///
/// * `src_dir` - the directory to archive
/// * `dest_dir` - the directory where the archive file should be placed
/// * `passphrase` - the passphrase to use for encryption
pub fn archive(src_dir: &Path, dest_dir: &Path, passphrase: &str) -> Result<PathBuf, io::Error> {
    let src_dir_canonical = fs::canonicalize(src_dir)?;
    let dest_dir_canonical = fs::canonicalize(dest_dir)?;
    let tar_file = tar(&src_dir_canonical, &dest_dir_canonical)?;
    let gpg_file = encrypt(&tar_file, &dest_dir_canonical, passphrase)?;
    fs::remove_file(&tar_file)?;
    Ok(gpg_file)
}

/// Decrypts and unpacks an archive file created by `archive`.
///
/// # Arguments
///
/// * `src_file` - the archive file
/// * `dest_dir` - the directory in which the unarchived directory will be placed
/// * `passphrase` - the passphrase for decryption
pub fn unarchive(src_file: &Path, dest_dir: &Path, passphrase: &str) -> Result<PathBuf, io::Error> {
    let src_file_canonical = fs::canonicalize(src_file)?;
    let dest_dir_canonical = fs::canonicalize(dest_dir)?;
    let tar_file = decrypt(&src_file_canonical, &dest_dir_canonical, passphrase)?;
    let unarchived_dir = untar(&tar_file, &dest_dir_canonical)?;
    fs::remove_file(&tar_file)?;
    Ok(unarchived_dir)
}

fn tar(src_dir: &Path, dest_dir: &Path) -> Result<PathBuf, io::Error> {
    let base_dir = src_dir.parent().expect("unable to get parent directory");
    let item_name = src_dir.file_name()
        .expect("unable to get directory name")
        .to_str()
        .expect("non-unicode path");
    let tar_bz_filename = format!("{}.tar.bz2", item_name);
    let dest_file = dest_dir.join(tar_bz_filename);
    let output = Command::new("tar")
        .args(&["-cjf", path_to_str(&dest_file),
                "-C", path_to_str(base_dir),
                item_name])
        .output()?;
    assert!(output.status.success());
    assert!(dest_file.is_file());
    Ok(dest_file)
}

fn untar(tar_file: &Path, dest_dir: &Path) -> Result<PathBuf, io::Error> {
    let output = Command::new("tar")
        .args(&["-C", path_to_str(dest_dir),
                "-xf", path_to_str(tar_file)])
        .output()?;
    assert!(output.status.success());
    let unarchived_dir_name = String::from(tar_file.file_name().expect("unable to get file name")
                                                   .to_str().expect("unable to convert to string"))
        .replace(".tar.bz2", "");
    let unarchived_dir = dest_dir.join(unarchived_dir_name);
    assert!(unarchived_dir.is_dir());
    Ok(unarchived_dir)
}

fn encrypt(tar_file: &Path, dest_dir: &Path, passphrase: &str) -> Result<PathBuf, io::Error> {
    let item_name = tar_file.file_name()
        .expect("unable to get tar file name")
        .to_str()
        .expect("non-unicode path");
    let gpg_filename = format!("{}.gpg", item_name);
    let dest_file = dest_dir.join(gpg_filename);
    let output = Command::new("gpg")
        .args(&["--batch",
                "--symmetric",
                "--passphrase", passphrase,
                "-o", path_to_str(&dest_file),
                "--cipher-algo", CIPHER,
                path_to_str(tar_file)])
        .output()?;
    assert!(output.status.success());
    assert!(dest_file.is_file());
    Ok(dest_file)
}

fn decrypt(src_file: &Path, dest_dir: &Path, passphrase: &str) -> Result<PathBuf, io::Error> {
    let dest_file = dest_dir.join(src_file.file_stem().expect("unable to get file stem"));
    let output = Command::new("gpg")
        .args(&["--batch",
                "--decrypt",
                "--passphrase", passphrase,
                "-o", path_to_str(&dest_file),
                "--cipher-algo", CIPHER,
                path_to_str(src_file)])
        .output()?;
    assert!(output.status.success());
    assert!(dest_file.is_file());
    Ok(dest_file)
}

fn path_to_str(path: &Path) -> &str {
    path.to_str().expect("non-unicode path")
}


#[cfg(test)]
mod tests {
    use super::*;

    extern crate tempdir;

    #[test]
    fn archive_round_trip() -> Result<(), io::Error> {
        let src_dir_name = "src-dir";
        let src_file_name = "file.txt";
        let test_content = "Archiving test";
        let test_passphrase = "Some secret passphrase";

        // set up the test directory
        let test_dir = tempdir::TempDir::new("archive_round_trip")?;
        let src_dir = test_dir.path().join(src_dir_name);
        fs::create_dir(&src_dir)?;
        fs::write(src_dir.join(src_file_name), test_content)?;
        let dest_dir = test_dir.path().join("dest-dir");
        fs::create_dir(&dest_dir)?;

        let archive_file = archive(&src_dir, &dest_dir, test_passphrase)?;

        // check that archive can be unpacked and has the expected contents
        let unpack_dir = test_dir.path().join("unpack-dir");
        fs::create_dir(&unpack_dir)?;
        let unarchived_dir = unarchive(&archive_file, &unpack_dir, test_passphrase)?;
        assert_eq!(unarchived_dir, unpack_dir.join(src_dir_name));
        let expected_unpacked_file = unarchived_dir.join(src_file_name);
        assert!(expected_unpacked_file.is_file());
        let read_content = fs::read_to_string(expected_unpacked_file)?;
        assert_eq!(read_content, test_content);
        Ok(())
    }
}
