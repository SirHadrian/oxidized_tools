use log::debug;
use md5::Digest;
use std::{
    collections::HashSet,
    ffi,
    ffi::OsString,
    fs::{self, remove_file, ReadDir},
    io,
};

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

// Insert files in hashset and return duplicates
pub fn find_duplicates(
    paths: ReadDir,
    dir: &String,
    mut hash_set: HashSet<Digest>,
) -> Vec<OsString> {
    let mut duplicate_list: Vec<OsString> = Vec::new();

    for file in paths {
        let file = file.expect("Failed to unwrap file from paths");

        if file
            .metadata()
            .expect("Failed to get metadata from file")
            .is_dir()
        {
            continue;
        }

        let full_name = build_full_file_path(dir, &file.file_name());
        debug!("Full file name: {:?}", full_name);
        let md5_sum =
            calcuate_md5_sum(&full_name).expect("Could not open file to calculate the md5 sum");

        if hash_set.contains(&md5_sum) {
            duplicate_list.push(full_name.clone());
            println!("\nFile_name: {:?}", full_name);
            println!("Md5: {:?}", md5_sum);
        } else {
            hash_set.insert(md5_sum);
        }
    }
    duplicate_list
}

fn build_full_file_path(directory: &String, file: &OsString) -> OsString {
    let mut full_name = OsString::new();
    full_name.push(directory);
    // Add backslash if the dir dosen't have one
    if !directory.ends_with("/") {
        full_name.push("/");
    }
    full_name.push(file);

    full_name
}

pub fn move_file(file: &OsString, path: &String) {
    // Copy file
    fs::copy(file, path).expect("Could not coppy the file");

    // Delete file
    delete_file(file);
}

pub fn delete_file(file: &OsString) {
    debug!("Deleting file: {:?}", file);
    if let Err(e) = remove_file(file) {
        eprintln!("Could not delete the file: {}", e);
    }
}

pub fn help() {
    println!("\nFind duplicates");
    println!("\nScan directory and find duplicates");
    println!("\nUsage: [EXE] [OPTIONS] -d | --dir <DIRECTORY>");
    println!("\nOptions:");
    println!("-h, --help            Print help");
    println!("-m, --move <DIR>      Move the duplicate files in the supplied direcotry, cannot with --delete");
    println!("--delete              Delete duplicates, cannot use with -m, --move");
}
