use proconio::input;

fn main() {
    input! {
        n: usize,
        tlr: [(usize, usize, usize); n]
    }

    let mut ans = 0;
    for i in 0..n-1 {
        for j in i+1..n {
            let l1 = if tlr[i].0 <= 2 { 2 * tlr[i].1 } else { 2 * tlr[i].1 + 1 };
            let r1 = if tlr[i].0 % 2 == 1 { 2 * tlr[i].2 } else { 2 * tlr[i].2 - 1 };
            let l2 = if tlr[j].0 <= 2 { 2 * tlr[j].1 } else { 2 * tlr[j].1 + 1 };
            let r2 = if tlr[j].0 % 2 == 1 { 2 * tlr[j].2 } else { 2 * tlr[j].2 - 1 };

            // println!("{} {} / {} {}", l1, r1, l2, r2);
            if l1 <= l2 && l2 <= r1 { ans += 1 }
            else if l2 <= l1  && l1 <= r2 { ans += 1 }
        }
    }
    println!("{}", ans);
}