use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n]
    }

    let init = k / n;
    let lim = k % n;

    // 国民の持っている個数
    let mut ans = vec![init; n];

    if lim != 0 {
        // 国民番号いくつ以下がもらえるか調べる
        let mut aa = a.clone();
        aa.sort();
        let kokuban = aa[lim - 1];
        for i in 0..n {
            if a[i] <= kokuban {
                ans[i] += 1;
            }
        }
    }

    for ai in ans {
        println!("{:?}", ai);
    }
}