use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        d: usize,
        xy: [(usize, usize); n]
    }

    for (x, y) in xy {
        if x < s && y > d {
            println!("Yes");
            return
        }
    }
    println!("No");
}