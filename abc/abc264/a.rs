use proconio::input;

fn main() {
    input! {
        l: usize,
        r: usize,
    }
    let s: Vec<char> = "atcoder".chars().collect();
    for i in l - 1..=r - 1 {
        print!("{}", s[i]);
    }
    println!("");
}
