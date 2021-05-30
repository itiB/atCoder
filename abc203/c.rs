use proconio::input;

fn main() {
    input! {
        n: usize,
        mut k: i128,
        mut ab: [(i128, i128); n]
    }

    ab.sort_by_key(|x| x.0);
    for (a, b) in ab {
        if a <= k {
            k += b;
        } else {
            break;
        }
    }
    println!("{}", k);
}