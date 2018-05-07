extern crate assert_cli;

//kcov doesn't play nice with assert_cli() see
//https://github.com/assert-rs/assert_cli/issues/101
use std::env;
fn get_cwd() -> String {
    env::current_dir().unwrap().to_str().unwrap().to_string()
}

#[test]
fn test_help() {
    //test that help works contains a USAGE string
    let bin: &str = &format!("{}/target/debug/device-checkout", get_cwd());
    assert_cli::Assert::command(&[bin])
        .with_args(&["-h"])
        .stdout()
        .contains("USAGE")
        .unwrap();
}

#[test]
fn test_verbosity_limit() {
    //test that we error out with too many verbosity flags
    let bin: &str = &format!("{}/target/debug/device-checkout", get_cwd());
    assert_cli::Assert::command(&[bin])
        .with_args(&["-vvvvv"])
        .fails()
        .unwrap();
}

#[test]
fn test_timestamp_parsing() {
    //test that a bogus timestamp errors out
    let bin: &str = &format!("{}/target/debug/device-checkout", get_cwd());
    assert_cli::Assert::command(&[bin])
        .with_args(&["-t", "bogus"])
        .fails()
        .unwrap();
}
