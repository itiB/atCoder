use proconio::input;
use std::collections::HashMap;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut map: HashMap<usize, usize> = HashMap::new();

    for i in 1..1 << n {
        let mut sum = 0;
        for j in 0..n {
            if i >> j & 1 == 1 {
                // j bit目が立っているならば
                sum += a[j];
                sum %= 200;
            }
        }
        if map.contains_key(&(sum)) {
            let val = map.get(&(sum)).unwrap(); // 過去のその数値になったやつ
            let (c1, v1) = print_bits(*val, n);
            let (c2, v2) = print_bits(i, n);
            for n in 0..min(c1, c2) {
                if v1[n] != v2[n] {
                    print_ans(c1, v1, c2, v2);
                    return
                }
            }
        } else {
            map.insert(sum, i);
        }
    }
    println!("No");
}

fn print_ans(c1: usize, v1: Vec<usize>, c2: usize, v2: Vec<usize>) {
    println!("Yes");
    print!("{} ", c1);
    for n in v1 {
        print!("{} ", n + 1);
    }
    println!("");
    print!("{} ", c2);
    for n in v2 {
        print!("{} ", n + 1);
    }
    println!("");
}

fn print_bits(i: usize, n: usize) -> (usize, Vec<usize>) {
    let mut count = 0;
    let mut v = Vec::new();
    for j in 0..n {
        if i >> j & 1 == 1 {
            v.push(j);
            count += 1;
        }
    }
    (count, v)
}