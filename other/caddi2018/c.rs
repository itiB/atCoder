use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        p: usize,
    }
    let fact = factor(p);
    let mut ans = 1;
    for (key, val) in fact {
        if val >= n {
            ans = ans * key.pow((val / n) as u32);
        }
    }
    println!("{}", ans);
}

fn factor(num: usize) -> HashMap::<usize, usize> {
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
