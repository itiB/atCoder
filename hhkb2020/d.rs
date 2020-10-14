use proconio::input;

const MOD: i64 = 1_000_000_007;

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: i64,
            a: i64,
            b: i64,
        }

        // let mut split = 0;
        // for x in b..n - a {
        //     split += (n - a + 1) * (n - b + 1) * (x - b + 1)
        // }
        // split の値は x のみが変数
        // x' = x - b + 1;
        // x': (1 ~ n - a - b + 1)
        // split はこの範囲の和
        // 1 ~ r の和: (r + 1) * r / 2

        // let mut cross = 0;
        // for x in b..n - a {
        //     for y in b..n - a {
        //         divisor += (x - b + 1) * (y - b + 1);
        //     }
        // }
        // sig(sig(r(x) * r'(y))) = sig(r(x))sig(r'(y))
        // x - b + 1 -> 1 ~ n - a - b + 1
        // y - b + 1 -> 1 ~ n - a - b + 1

        let xb1 = if n >= a + b { (n - a - b + 2) * (n - a - b + 1) / 2 % MOD } else { 0 };
        let split = xb1 * (n - b + 1) % MOD * (n - a + 1) % MOD;
        let cross = xb1 * xb1 % MOD;

        println!("{}", (split * 4 % MOD - cross * 4 % MOD + MOD) % MOD);
    }
}
