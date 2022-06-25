use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize
    }
    let map: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();

    println!("{}", map[(x - 1) / n]);
}
