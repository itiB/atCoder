use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }

    let mut count = 0;

    for num in 0..n {
        if (num + 1) % 2 != 0 {
            if a[num] % 2 != 0 {
                count += 1;
            }
        }
    }

    println!("{}", count);
}