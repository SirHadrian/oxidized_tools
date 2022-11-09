use md5::Digest;
use std::{
    collections::HashSet,
    ffi,
    ffi::OsString,
    fs::{self, ReadDir},
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
pub fn find_duplicates(paths: ReadDir, mut hash_set: HashSet<Digest>) -> Vec<OsString> {
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

        let file_name = file.file_name();
        let md5_sum =
            calcuate_md5_sum(&file_name).expect("Could not open file to calculate the md5 sum");

        if hash_set.contains(&md5_sum) {
            duplicate_list.push(file_name.clone());
            println!("File name: {:?}, md5: {:?}", file_name, md5_sum);
        } else {
            hash_set.insert(md5_sum);
        }
    }
    duplicate_list
}

pub fn move_file(file: &OsString, new_path: String) {
    // Copy file
    fs::copy(file, new_path).expect("Could not coppy the file");

    // Delete file
}
