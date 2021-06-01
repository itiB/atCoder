use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    for i in (1..=b-a).rev() {
        let j = (a -1) / i + 1;
        if a <= i * j && i * j <= b {
            if a <= i * (j + 1) && i * (j + 1) <= b {
                println!("{}", i);
                return
            }
        }
    }
    println!("1");
}
