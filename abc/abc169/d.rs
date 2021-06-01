use proconio::input;

fn main() {
    input! {
        n: i64,
    }

    let mut ans = 0;
    for (_, count) in factor(n) {
        let mut n = count;
        let mut b = 1;
        while b <= n {
            n -= b;
            b += 1;
            ans += 1;
        }
    }
    println!("{}", ans);
}

// O(n^(1/2)) で与えられた値の素因数分解を求める
use std::collections::HashMap;

fn factor(num: i64) -> HashMap::<i64, i64> {
    let mut map = HashMap::new();
    let mut n = num;

    let mut i = 2;
    loop {
        if n == 1 { 
            // 割り切れたら抜ける
            break;
        }
        if i * i >= num {
            // n^(1/2)まで試す
            // 残っていたら追加して抜ける
            map.insert(n, 1);
            break;
        }
        if n % i == 0 {
            // Mapに無ければ追加、あれば1増やす
            *map.entry(i).or_insert(0) += 1;
            n /= i;
            continue;
        }
        i += 1;
    }
    map
}
