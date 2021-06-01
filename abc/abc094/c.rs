use proconio::input;

fn main() {
    input! {
        n: usize,
        x: [i32; n],
    }

    let mut sort = x.clone();
    sort.sort();
    let mid = (n - 1) / 2;

    for i in 0..n {
        let m = if x[i] <= sort[mid] { mid + 1 } else { mid };
        println!("{}", sort[m]);
    }
}