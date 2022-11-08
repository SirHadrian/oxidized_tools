use env_logger;
use log::debug;
use md5::Digest;
use std::{collections::HashSet, env, fs, io, ffi};

// Get all the files in a directory
fn get_files_from_dir(path: &str) -> Result<fs::ReadDir, io::Error> {
    Ok(fs::read_dir(path)?)
}
// Calculate md5 sum for one file;
fn calcuate_md5_sum(file_name: ffi::OsString)->Result<Digest, io::Error>{
    let file =fs::read(file_name)?;
    let md5_sum=md5::compute(file);

    Ok(md5_sum)
}
// Insert the file in hashset
// Put the duplicates in a separate list

fn main() {
    env_logger::init();

    let mut args = env::args().skip(1);
    debug!("Supplied arguments: {:?}", args);

    let test_dir = args.next().expect("Directory not supplyed");
    debug!("Supplied direcotry: {}", test_dir);
}
