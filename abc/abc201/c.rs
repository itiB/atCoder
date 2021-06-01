use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }
    let mut ans = 0;
    for i in 0..10000 {
        let mut flag = [false; 10];
        let mut x = i;
        let mut tmp = true;
        for _ in 0..4 {
            flag[x % 10] = true;
            x /= 10;
        }
        for i in 0..10 {
            if s[i] == 'o' && flag[i] == false { tmp = false; }
            if s[i] == 'x' && flag[i] == true {tmp = false; }
        }
        if tmp == true {
            ans += 1;
        }
    }
    println!("{}", ans);
}