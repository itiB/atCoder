use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }

    if a == b + c + d
        || b == a + c + d
        || c == a + b + d
        || d == a + b + c
        || a + b == c + d
        || a + c == b + d
        || a + d == b + c
    {
        println!("Yes");
    } else {
        println!("No");
    }
}
