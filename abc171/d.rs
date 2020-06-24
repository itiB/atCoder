use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i32; n],
        q: usize,
        bc: [(i32, i32); q],
    }

    for (b, c) in bc {
        let mut sum = 0;
        for l in 0..a.len() {
            if a[l] == b {
                a[l] = c;
            }
            sum += a[l];
        }
        println!("{}", sum);
    }
}