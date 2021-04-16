use assert_cmd::Command;

#[test]
fn trivial() {
    let output = Command::cargo_bin("spellit")
        .unwrap()
        .arg("Hello 1337!")
        .unwrap();
    assert!(output.status.success());
    assert_eq!(
        "H -> Hotel\ne -> Echo\nl -> London\nl -> London\no -> Oscar\n  -> \n1 -> One\n3 -> Three\n3 -> Three\n7 -> Seven\n! -> \n",
        String::from_utf8(output.stdout).unwrap()
    );
}

#[test]
fn from_stdin() {
    let output = Command::cargo_bin("spellit")
        .unwrap()
        .write_stdin("Hello 1337!")
        .unwrap();
    assert!(output.status.success());
    assert_eq!(
        "H -> Hotel\ne -> Echo\nl -> London\nl -> London\no -> Oscar\n  -> \n1 -> One\n3 -> Three\n3 -> Three\n7 -> Seven\n! -> \n",
        String::from_utf8(output.stdout).unwrap()
    );
}
