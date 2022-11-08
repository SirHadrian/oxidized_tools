use env_logger;
use log::debug;
use md5::Digest;
use std::{collections::HashSet, env, fs, io};

// Get all the files in a directory
// Calculate md5 sum for one file
// Insert the file in hashset
// Put the duplicates in a separate list

fn main() {
    env_logger::init();

    let mut args = env::args().skip(1);
    debug!("Supplied arguments: {}", args);



}
