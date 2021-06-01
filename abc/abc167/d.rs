use proconio::input;

fn main() {
    input! {
        n: usize,
        mut k: usize,
        a: [usize; n],
    }

    let mut s = Vec::new();
    let mut ord: Vec<i64> = vec![-1; n + 1]; // 何回目のジャンプでそこに辿りつくか

    let mut go: usize = 1;
    loop {
        if ord[go] != -1 {
            break;
        }
        ord[go] = s.len() as i64;
        s.push(go);
        go = a[go - 1];
    }
    let loop_size = s.len() - ord[go] as usize; // 周期
    let l = ord[go] as usize; // 例外部分の長さ

    if k < l {
        println!("{}", s[k]);
    } else {
        k -= l;
        k %= loop_size;
        println!("{}", s[l + k]);
    }
}