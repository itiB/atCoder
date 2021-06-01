use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"1 2 3
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "3 1 2\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"100 100 100
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "100 100 100\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"41 59 31
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "31 41 59\n");
    assert!(output.stderr_str().is_empty());
}
