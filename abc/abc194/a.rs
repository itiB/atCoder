use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize
    }

    println!("{}", if b >= 8 && a + b >= 15 {
        1
    } else if b >= 3 && a + b >= 10 {
        2
    } else if a + b >= 3 {
        3
    } else {
        4
    })
}
