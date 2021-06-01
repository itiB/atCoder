use proconio::input;

fn main() {
    input! {
        mut a: usize,
        mut b: usize,
        c: usize,
    }

    if a + c > b {
        println!("Takahashi");
    } else {
        println!("Aoki");
    }
}