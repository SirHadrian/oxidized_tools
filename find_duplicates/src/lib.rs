use std::{fs, io, ffi};
use md5::Digest;

// Get all the files in a directory
pub fn get_files_from_dir(path: &str) -> Result<fs::ReadDir, io::Error> {
    Ok(fs::read_dir(path)?)
}

// Calculate md5 sum for one file;
pub fn calcuate_md5_sum(file_name: &ffi::OsString) -> Result<Digest, io::Error> {
    let file = fs::read(file_name)?;
    let md5_sum = md5::compute(file);

    Ok(md5_sum)
}
