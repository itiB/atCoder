use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        u: usize,
        a: [usize; u],
    } 
    let mut map = HashMap::new();
 
    for &a in a.iter() {
        *map.entry(a).or_insert(0) += 1;
    }
    let key = map.keys().collect::<Vec<_>>();

    // 中身がすべて0
    if map.len() == 1 {
        if let Some(_) = map.get(&0) {
            println!("Yes");
        } else {
            println!("No");
        }

    // 中身の1/3が0と2/3が何かしらの同じ値
    } else if map.len() == 2 {
        if u % 3 == 0 && map.get(&0) == Some(&(u / 3)) && key.len() == 2 {
            println!("Yes");
        } else {
            println!("No");
        }

    // 中身の値がx ^ y ^ z = 0 かつそれぞれ1/3づつ含まれている
    } else if map.len() == 3 {
        if map.len() == 3 {
            if u % 3 == 0 && map.values().all(|&c| c == u / 3) && 
                key[0] ^ key[1] == *key[2] {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    } else {
        println!("No");
    }
}