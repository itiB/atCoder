use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let mut stack = Vec::new();
    let mut ans = 0;

    for c in s {
        if stack.len() == 0 {
            stack.push(c);
            continue;
        }

        if stack[stack.len() - 1] == c {
            stack.push(c);
        } else {
            stack.pop();
            ans += 2;
        }
    }
    println!("{}", ans);
}
