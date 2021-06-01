use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;

fn main() {
    input! {
        s: Chars
    }
    let mut map: HashMap<char, usize> = HashMap::new();

    for &a in s.iter() {
        *map.entry(a).or_insert(0) += 1;
        if *map.get(&a).unwrap() > 3 {
            *map.get_mut(&a).unwrap() = 3;
        }
    }
    let mut nums = Vec::new();
    for (key, val) in map {
        for _ in 0..val {
            nums.push(key);
        }
    }
    // println!("{:?}", nums);
    if nums.len() > 2 {
        for i in 0..nums.len() {
            for j in i+1..nums.len() {
                for v in j+1..nums.len() {
                    if (c2u(nums[i]) * 100 + (c2u(nums[j])) * 10 + c2u(nums[v])) % 8 == 0 ||
                    (c2u(nums[i]) * 100 + (c2u(nums[v])) * 10 + c2u(nums[j])) % 8 == 0 ||
                    (c2u(nums[j]) * 100 + (c2u(nums[i])) * 10 + c2u(nums[v])) % 8 == 0 ||
                    (c2u(nums[j]) * 100 + (c2u(nums[v])) * 10 + c2u(nums[i])) % 8 == 0 ||
                    (c2u(nums[v]) * 100 + (c2u(nums[i])) * 10 + c2u(nums[j])) % 8 == 0 ||
                    (c2u(nums[v]) * 100 + (c2u(nums[j])) * 10 + c2u(nums[i])) % 8 == 0 {
                        println!("Yes");
                        return
                    }
                }
            }
        }
    }
    if nums.len() == 2 {
        if (c2u(nums[1]) * 10 + c2u(nums[0])) % 8 == 0 ||
        (c2u(nums[0]) * 10 + c2u(nums[1])) % 8 == 0 {
            println!("Yes");
            return
        }
    } else {
        if c2u(nums[0]) % 8 == 0 {
            println!("Yes");
            return
        }
    }
    println!("No");
}

fn c2u(c: char) -> usize {
    c as usize - '0' as usize
}
