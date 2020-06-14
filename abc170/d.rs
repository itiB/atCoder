use proconio::input;

fn main() {
    input! {
        n: i32,
        a: [i64; n],
    }
    let mut count = 0;
    let a_it = a.clone();

    for num in 0..n {
        let sum = a_it.iter().filter(|s| a[num as usize] % *s == 0).count();
        if sum == 1 {
            count += 1;
        }
    }
    println!("{}", count);
}
