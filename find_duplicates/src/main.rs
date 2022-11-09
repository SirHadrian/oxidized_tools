use env_logger;
use find_duplicates::{delete_file, find_duplicates, get_files_from_dir, help, move_file};
use log::debug;
use md5::Digest;
use std::{collections::HashSet, env, process};

mod tests;

fn main() {
    env_logger::init();

    let mut args = env::args().skip(1);
    debug!("Supplied arguments: {:?}", args);

    // Set to keep all unique items
    let hash_set: HashSet<Digest> = HashSet::new();

    // Dir path
    let mut path: Option<String> = None;

    // Delete duplicate files after scan default: false
    let mut delete_files_flag = false;

    // Move path
    let mut move_path: Option<String> = None;
    let mut move_files_flag = false;

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
                help();
                process::exit(0);
            }
            "-m" | "--move" => match args.next() {
                Some(expr) => {
                    debug!("Supplied move path: {}", expr);
                    move_path = Some(expr);
                    move_files_flag = true;
                }
                None => {
                    eprintln!("No move path supplued");
                    process::exit(1);
                }
            },
            "--delete" => {
                delete_files_flag = true;
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

        if delete_files_flag{
            if move_files_flag{
                eprintln!("Cannot use delete flag with move flag");
                process::exit(1);
            }
            for file in &duplicate_list {
                delete_file(file);
            }
        }

        if move_files_flag{
            if delete_files_flag{
                eprintln!("Cannot use move flag with delete flag");
                process::exit(1);
            }
            let move_path = match move_path {
                Some(expr) => expr,
                None => {
                    eprintln!("No move path found");
                    process::exit(1);
                }
            };

            debug!("Move path: {}", move_path);
            for file in &duplicate_list {
                debug!("Moving file: {:?}", file);

                let move_file_path = format!(
                    "{}/{}",
                    move_path,
                    file.to_str().expect("Could not convert OsString to &str")
                );
                move_file(file, move_file_path);
            }
        }
    }
}
