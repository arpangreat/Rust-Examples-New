use std::process::{Command, Stdio};

#[test]
fn runs() {
    let mut cmd = Command::new("ls");
    let res = cmd.output();
    assert!(res.is_ok());
}

fn main() {
    let mut cmd = Command::new("ls");
    let res = cmd.output();
    cmd.stdout(Stdio::null());
    println!("{:?}", res);
}
