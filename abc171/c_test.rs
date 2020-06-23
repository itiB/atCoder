use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"2
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "b\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"27
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "aa\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"123456789
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "jjddja\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample4() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"26
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "z\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample5() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"52
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "az\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample6() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"475255
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "aaaaa\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample7() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"475254
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "zzzz\n");
    assert!(output.stderr_str().is_empty());
}
