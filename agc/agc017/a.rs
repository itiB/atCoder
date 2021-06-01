use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        p: usize,
        a: [usize; n],
    }

    let mut map: HashMap<usize, usize> = HashMap::new();
    map.insert(0, 0);
    map.insert(1, 0);

    for &a in a.iter() {
        *map.entry(a % 2).or_insert(0) += 1;
    }

    let ans = match p {
        0 => {
            cmb_all(map.get(&0).unwrap()) * cmb_even(map.get(&1).unwrap())
        },
        1 => cmb_all(map.get(&0).unwrap()) * cmb_odd(map.get(&1).unwrap()),
        _ => unreachable!(),
    };
    println!("{}", ans);
}

// すべての組み合わせを数え上げる
fn cmb_all(n: &usize) -> usize {
    let mut ans = 0;
    for num in 0..=*n {
        ans += combination(*n as i64, num as i64);
    }
    ans as usize
}

// 偶数個組み合わせた組み合わせを数える
fn cmb_even(n: &usize) -> usize {
    let mut ans = 0;
    for num in 0..=*n {
        if num % 2 == 0 {
            ans += combination(*n as i64, num as i64);
        }
    }
    ans as usize
}


// 奇数個組み合わせた組み合わせを数える
fn cmb_odd(n: &usize) -> usize {
    let mut ans = 0;
    for num in 0..=*n {
        if num % 2 == 1 {
            ans += combination(*n as i64, num as i64);
        }
    }
    ans as usize
}

fn combination(mut n: i64, k: i64) -> i64 {
    let mut result = 1;
    for i in 1..=k {
        result = result * n / i;
        n = n - 1;
    }
    result
}
