use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut table = vec![0; n];

    for x in 1..=100 {
        for y in 1..=100 {
            for z in 1..=100 {
                let num = x * x + y * y + z * z + x * y + y * z + z * x;
                if num <= n {
                    table[num - 1] += 1;
                }
            }
        }
    }
    for a in table {
        println!("{}", a);
    }
}