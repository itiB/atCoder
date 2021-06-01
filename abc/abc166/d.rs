use proconio::input;

fn main() {
    input! {
        n: i64,
    }

    for a in -119..=120 {
        for b in -119..=120 {
            if (a as i64).pow(5) - (b as i64).pow(5) == n {
                println!("{} {}", a, b);
                return
            }
        }
    }
}