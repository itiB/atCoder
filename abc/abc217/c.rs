use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n]
    }

    let mut ans = vec![0; n];
    for i in 0..n {
        ans[p[i]-1] = i+1;
    }
    for i in ans {
        print!("{} ", i);
    }
}