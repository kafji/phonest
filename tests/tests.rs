use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn example() {
    let output = Command::cargo_bin("spellit")
        .unwrap()
        .arg("Hello!")
        .unwrap();
    assert!(output.status.success());
    assert_eq!(
        "H -> Hotel\ne -> Echo\nl -> London\nl -> London\no -> Oscar\n! -> \n",
        String::from_utf8(output.stdout).unwrap()
    );
}
