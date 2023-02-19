
#[cfg(test)]
mod test {
    use std::process::Command;

    use assert_cmd::prelude::*;
    use predicates::prelude::*;

    /// Run test where two arguments are on the command line. This will fail
    /// and return 64 as the error code.
    #[test]
    fn test_two_args() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("rlox").unwrap();

        cmd.arg("1.txt")
            .arg("2.txt")
            .assert()
            .failure()
            .stdout(predicate::eq("Usage: rlox [script]\n"))
            .code(predicate::eq(64));

        Ok(())
    }
}

/*
run two more tests with these command lines:

cargo run --  ./scripts/oneline.lox 
should parse file.

cargo run
should start interactive mode. send it some commands.

 */