use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }

    let mut dp = vec![false; 100 * n + 1];
    dp[0] = true;

    for i in p {
        for j in (0..100*n).rev() {
            if dp[j] == true {
                dp[j + i] = true;
            }
        }
    }

    println!("{}", dp.iter().filter(|x| *x == &true).count());
}