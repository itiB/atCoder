use proconio::input;

fn main() {
    input! {
        n: i32,
        a: [i64; n],
    }
    let max = 1000001;

    let mut counter: Vec::<i64> = vec![0; max];

    for num in &a {
        let mut base = *num as usize;
        if counter[base] > 0 {
            counter[base] += 1;
            continue;
        }
        while base < max {
            counter[base] += 1;
            base += *num as usize;
        }
    }
    println!("{}", a.iter().filter(|n| counter[**n as usize] == 1).count());
}