extern crate assert_cli;

#[test]
fn test_basic() {
    //test that it runs
    assert_cli::Assert::main_binary().unwrap();
}

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

    //test that an acceptable number succeeds
    assert_cli::Assert::main_binary()
        .with_args(&["-vv"])
        .unwrap();
}

#[test]
fn test_timestamp_parsing() {
    //test that a bogus timestamp errors out
    assert_cli::Assert::main_binary()
        .with_args(&["-t", "bogus"])
        .fails()
        .unwrap();

    //and that a real timestamp succeeds
    assert_cli::Assert::main_binary()
        .with_args(&["-t", "sec"])
        .unwrap();

    //and that a long argument timestamp succeeds
    assert_cli::Assert::main_binary()
        .with_args(&["--timestamp", "ms"])
        .unwrap();
}
