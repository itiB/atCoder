use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }

    println!("{}",
        if a == b { c }
        else if b == c { a }
        else if c == a { b }
        else { 0 }
    );
}