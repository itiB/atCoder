use proconio::input;

fn main() {
    input! {
        n: usize,
        mut ab: [(usize, usize); n],
    }

    // bのキーでソートする
    // ab.sort_by(|a, b| a.1.cmp(&b.1));
    ab.sort_by_key(|ab| ab.1);

    let mut sum = 0;
    for (a, b) in ab {
        sum += a;
        if sum > b {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
