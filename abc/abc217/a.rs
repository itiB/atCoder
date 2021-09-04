use proconio::input;

fn main() {
    input! {
        s: [String; 2],
    }
    let mut tmp = s.clone();
    tmp.sort();

    println!("{}", if tmp[0] == s[0] { "Yes" } else { "No" });
}