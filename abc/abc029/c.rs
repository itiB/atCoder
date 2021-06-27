use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    printer(n, "".to_string());
}

fn printer(r: usize, s: String) {
    if r == 0 {
        println!("{}", s);
    } else {
        printer(r - 1, s.clone() + "a");
        printer(r - 1, s.clone() + "b");
        printer(r - 1, s.clone() + "c");
    }
}