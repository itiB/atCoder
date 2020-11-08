use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: Chars,
    }

    let mut num = Vec::new();
    let mut sum = 0;
    for c in n {
        let u = c2u(c);
        num.push(u % 3);
        sum += u;
    }
    if sum % 3 == 0 {
        println!("0");
    } else if sum % 3 == 1 {
        if num.contains(&1) && num.len() > 1 {
            println!("1");
        } else {
            let count = num.iter().fold(0, |sum, x| sum + if x == &2 { 1 } else { 0 });
            if count > 1 && num.len() > 2 {
                println!("2");
            } else {
                println!("-1");
            }
        }
    } else if sum % 3 == 2 {
        if num.contains(&2) && num.len() > 1 {
            println!("1");
        } else {
            let count = num.iter().fold(0, |sum, x| sum + if x == &1 { 1} else { 0 });
            if count > 1 && num.len() > 2 {
                println!("2");
            } else {
                println!("-1");
            }
        }
    }
}

fn c2u(c: char) -> usize {
    c as usize - '0' as usize
}