use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [u128; n],
    }

    for num in 0..n - k{
        if a[num] < a[num + k] {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}