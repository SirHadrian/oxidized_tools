
#[cfg(test)]
pub mod test{

    use std::error::Error;
    use std::process::Command;
    use assert_cmd::prelude::*;

    #[test]
    fn run_binary_without_flags() ->Result<(), Box<dyn Error>>{

        Command::cargo_bin("find_duplicates").expect("Binary exist").assert().success();

        Ok(())
        
    }

    #[test]
    fn fail_on_no_dir()->Result<(), Box<dyn Error>>{

        Command::cargo_bin("find_duplicates").expect("Binary exist").args(&["-d", "no/such/file.txt", "--dir", "no/such/file.txt"]).assert().failure();

        Ok(())
    }
    
}
