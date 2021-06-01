use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
        k: usize,
    }

    for i in 0..=k {
        for j in 0..=k {
            if i + j > k { break; }
            for v in 0..=k {
                if i + j + v > k { break; }
                if a * 1 << i < b * 1 << j && c * 1 << v > b * 1 << j {
                    println!("Yes");
                    return;
                }
            }
        }
    }
    println!("No");
}
