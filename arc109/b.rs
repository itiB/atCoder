use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut ans = n + 1;
    let mut cut = 1;
    let mut remain = n + 1;
    while cut <= remain {
        remain -= cut;
        cut += 1;
        ans -= 1;
    }
    println!("{}", ans);
}