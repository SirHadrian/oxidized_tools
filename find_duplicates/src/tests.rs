#[cfg(test)]
pub mod test {

    use assert_cmd::prelude::*;
    use std::error::Error;
    use std::process::Command;

    #[test]
    fn run_binary_without_flags() -> Result<(), Box<dyn Error>> {
        Command::cargo_bin("find_duplicates")
            .expect("Binary exist")
            .assert()
            .success();

        Ok(())
    }

    #[test]
    fn fail_no_path() -> Result<(), Box<dyn Error>> {
        Command::cargo_bin("find_duplicates")
            .expect("Binary exist")
            .args(&["-d", "no/such/file.txt"])
            .assert()
            .failure();
        Command::cargo_bin("find_duplicates")
            .expect("Binary exist")
            .args(&["--dir", "no/such/file.txt"])
            .assert()
            .failure();

        Ok(())
    }
}
