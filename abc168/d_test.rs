use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"4 4
1 2
2 3
3 4
4 2
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "Yes
1
2
2
\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"6 9
3 4
6 1
2 4
5 3
4 6
1 5
6 2
4 5
5 6
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "Yes
6
5
5
1
1
\n");
    assert!(output.stderr_str().is_empty());
}
