use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        k: usize
    }

    let mut ans = 0;
    let mut ss = s.clone();

    if s[0] != s[s.len() - 1] {
        // はじめと最後が違う文字 -> そのまま掛け算
        for i in 1..ss.len() {
            if ss[i - 1] == ss[i] {
                ans += 1;
                ss[i] = '.';
            }
        }
        ans *= k;
    } else {
        ss.append(&mut s.clone());
        let mut mid = 0;
        for i in 1..ss.len() {
            if ss[i - 1] == ss[i] {
                ans += 1;
                if i < s.len() { mid += 1; }
                ss[i] = '.';
            }
        }
        if ss[0] != ss[ss.len() - 1] {
            // 2回まとめたらそのままつなげられる -> 奇数回なら調整
            ans *= k / 2;
            if k % 2 == 1 {
                ans += mid
            }
        } else {
            // 2回目以降は全部同じ数が足されるパターン
            ans -= mid;
            ans *= k - 1;
            ans += mid;
        }
    }
    println!("{}", ans);
}