use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        q: usize,
        mut a: [usize; k],
        l: [usize; q]
    }
    for i in 1..=q {
        let target = l[i - 1] - 1; // 番目の駒
                                   // println!("{}, target={}, a[target]={}", i, target, a[target]);
        if a[target] < n {
            // println!(" in target={}, a[target]={}", target, a[target]);
            if target < k - 1 {
                if a[target] + 1 != a[target + 1] {
                    // println!("  1");
                    a[target] += 1;
                }
                // 隣に駒があるならなにもしない
            } else {
                // 一番右の駒ならば進めるだけ
                // println!("  2");
                a[target] += 1;
            }
        }
        // println!("pass {:?}", a);
        // 端まで到達していたら何もしない
    }
    for i in a {
        print!("{} ", i);
    }
    println!("");
}
