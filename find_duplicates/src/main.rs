use env_logger;
use find_duplicates::{ get_files_from_dir, find_duplicates};
use log::debug;
use md5::Digest;
use std::{collections::HashSet, env};

fn main() {
    env_logger::init();

    let mut args = env::args().skip(1);
    debug!("Supplied arguments: {:?}", args);

    let test_dir = args.next().expect("Directory not supplyed");
    debug!("Supplied direcotry: | {} |", test_dir);

    let hash_set: HashSet<Digest> = HashSet::new();

    let paths = get_files_from_dir(&test_dir).expect("Could not get the files from direcotry");

    find_duplicates(paths, hash_set);
}
