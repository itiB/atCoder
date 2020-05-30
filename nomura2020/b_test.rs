use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"PD?D??P
"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "PDPDPDP\n");
    assert!(output.stderr_str().is_empty());
}
 #[test]
 fn sample2() {
     let testdir = TestDir::new(BIN, "");
     let output = testdir
         .cmd()
         .output_with_stdin(r#"P?P?
 "#)
         .tee_output()
         .expect_success();
     assert_eq!(output.stdout_str(), "PDPD\n");
     assert!(output.stderr_str().is_empty());
 }

 #[test]
 fn sample3() {
     let testdir = TestDir::new(BIN, "");
     let output = testdir
         .cmd()
         .output_with_stdin(r#"????
 "#)
         .tee_output()
         .expect_success();
     assert_eq!(output.stdout_str(), "PDPD\n");
     assert!(output.stderr_str().is_empty());
 }
 