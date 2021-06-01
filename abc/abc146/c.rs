use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        x: i64,
    }

    let mut left: i64 = 0;
    let mut right: i64 = 1_000_000_001;

    while right - left > 1 {
        // 2分探索
        // 全部の値を調べ終わるまで
        let mid = (left + right) / 2;
        if a * mid + b * num_digit(mid) <= x {
            // 値が買える範囲内ならば左を寄せる
            left = mid;
        } else {
            // 値が買えないなら右を寄せて買えるとこを探す
            right = mid;
        }
    }
    println!("{}", left);
}

fn num_digit<T: std::string::ToString>(x: T) -> i64 {
    let s: Vec<char> = x.to_string().chars().collect();
    s.len() as i64
}
