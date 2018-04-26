extern crate assert_cli;

#[test]
fn test_help() {
    //test that help works contains a USAGE string
    assert_cli::Assert::main_binary()
        .with_args(&["-h"])
        .stdout()
        .contains("USAGE")
        .unwrap();
}

#[test]
fn test_verbosity_limit() {
    //test that we error out with too many verbosity flags
    assert_cli::Assert::main_binary()
        .with_args(&["-vvvvv"])
        .fails()
        .unwrap();
}

#[test]
fn test_timestamp_parsing() {
    //test that a bogus timestamp errors out
    assert_cli::Assert::main_binary()
        .with_args(&["-t", "bogus"])
        .fails()
        .unwrap();
}
