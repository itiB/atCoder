use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        mut n: usize
    }

    let mut nums = Vec::new();
    loop {
        nums.push(n % 10);
        if n < 10 {
            break
        }
        n /= 10;
    }

    let mut ans = 0;
    for i in 1..1usize << nums.len() {
        let mut sum1 = 0;
        let mut num1 = Vec::new();
        let mut sum2 = 0;
        let mut num2 = Vec::new();
        for j in 0..nums.len() {
            if i >> j & 1 == 1 {
                sum1 += nums[j];
                num1.push(nums[j])
            } else {
                sum2 += nums[j];
                num2.push(nums[j])
            }
        }
        if sum1 == 0 || sum2 == 0 { continue; }
        ans = max(ans, generate_num(&mut num1) * generate_num(&mut num2));
    }
    println!("{}", ans);
}

fn generate_num(nums: &mut [usize]) -> usize {
    nums.sort();
    let mut tmp = 1;
    let mut ans = 0;
    for i in 0..nums.len() {
        ans += nums[i] * tmp;
        tmp *= 10;
    }
    ans
}