use env_logger;
use log::debug;
use md5::Digest;
use std::{collections::HashSet, env};
use find_duplicates::{get_files_from_dir, calcuate_md5_sum};
// Insert the file in hashset
// Put the duplicates in a separate list

fn main() {
    env_logger::init();

    let mut args = env::args().skip(1);
    debug!("Supplied arguments: {:?}", args);

    let test_dir = args.next().expect("Directory not supplyed");
    debug!("Supplied direcotry: {}", test_dir);

    let mut hash_set: HashSet<Digest> = HashSet::new();

    let paths = get_files_from_dir(&test_dir).expect("Could not get the files from direcotry");

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
            println!("{:?}", file_name);
        } else {
            hash_set.insert(md5_sum);
        }
    }
}
