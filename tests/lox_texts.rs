/// Tests for the lox interpreter

#[cfg(test)]
mod test {
    use assert_cmd::cmd::Command;
    use predicates::prelude::*;

    /// Run test where two arguments are on the command line. This will fail
    /// and return 64 as the error code. The command should be like this:
    /// ```
    /// :~$ rlox 1.txt 2.txt
    /// ```
    /// Output should look like this:
    /// ```
    /// Usage: rlox [script]
    /// ```
    /// 
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

    /// Run test where the interpreter reads a file, like this:
    /// ```
    /// :~$ rlox ./scripts/oneline.lox
    /// ```
    /// This is my sample file.
    /// 
    /// There should be some output.
    /// 
    /// Not doing a full test since this will change a lot.
    #[test]
    fn test_read_one_line_file() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("rlox").unwrap();

        cmd.arg("./scripts/oneline.lox")
            .assert()
            .success()
            .code(predicate::eq(0));

        Ok(())
    }

    /// Run test with no arguments. This will open the live interpreter.
    /// Send it some input. Verify it gets some response.
    /// No full test yet since it will change a lot.
    #[test]
    fn test_live_interpreter() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("rlox").unwrap();

        cmd
            .write_stdin("(\n")
            .write_stdin("\n")
            .assert()
            .stdout(predicate::eq("> "))
            .success()
            .code(predicate::eq(0));

        Ok(())
    }
}
