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
