use env_logger;
use find_duplicates::{find_duplicates, get_files_from_dir};
use log::debug;
use md5::Digest;
use std::{collections::HashSet, env, process};

fn main() {
    env_logger::init();

    let mut args = env::args().skip(1);
    debug!("Supplied arguments: {:?}", args);

    // Set to keep all unique items
    let hash_set: HashSet<Digest> = HashSet::new();

    // Dir path
    let mut path: Option<String> = None;

    loop {
        // Get the next arg from iterator
        let argument = match args.next() {
            Some(expr) => expr,
            None => break,
        };

        match argument.as_str() {
            "-d" | "--dir" => match args.next() {
                Some(value) => {
                    debug!("Supplied direcotry: | {} |", value);
                    path = Some(value);
                }
                None => {
                    eprintln!("No path value supplied");
                    process::exit(1);
                }
            },
            "-h" | "--help" => {
                process::exit(0);
            }
            _ => {
                eprintln!("Wrong argument type");
                process::exit(1);
            }
        }
    }
    
    if let Some(path) = path {
        let paths = get_files_from_dir(&path).expect("Could not get the files from direcotry");
        let duplicate_list = find_duplicates(paths, hash_set);
        debug!("Duplicate files: {:?}", duplicate_list);
    }
}
