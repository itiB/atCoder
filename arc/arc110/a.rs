use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut map = HashMap::new();
    for i in 2..=n {
        let mini = factor(i);
        for (key, val) in mini.into_iter() {
            let map_val = map.entry(key).or_insert(val);
            if *map_val < val {
                map.insert(key, val);
            }
        }
    }
    let mut ans = 1;
    println!("{:?}", map);
    for (key, val) in map {
        for _ in 0..val {
            ans *= key;
        }
    }
    while ans < n {
        ans += ans;
    }
    println!("{}", ans + 1);
}


use std::collections::HashMap;

fn factor(num: usize) -> HashMap::<usize, usize> {
    let mut map = HashMap::new();
    let mut n = num;

    let mut i = 2;
    loop {
        if n == 1 { 
            // 割り切れたら抜ける
            break;
        }
        if i * i > num {
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
