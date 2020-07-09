use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn example() {
    Command::cargo_bin("phonest")
        .unwrap()
        .args(&["Hello!"])
        .assert()
        .stdout(predicate::str::similar(
            "H -> Hotel\ne -> Echo\nl -> London\nl -> London\no -> Oscar\n! -> \n",
        ));
}

#[test]
fn with_no_arg() {
    Command::cargo_bin("phonest")
        .unwrap()
        .assert()
        .stderr(predicate::str::similar(
            "Require 1 string argument. e.g. 'Hello world!'\n",
        ));
}
