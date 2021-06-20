use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        sa: Chars,
        sb: Chars,
        sc: Chars,
    }

    let mut a = 0;
    let mut b = 0;
    let mut c = 0;

    let mut turn = 'a';
    loop {
        match turn {
            'a' => {
                if a == sa.len() {
                    println!("A");
                    break
                }
                turn = sa[a];
                a += 1;
            },
            'b' => {
                if b == sb.len() {
                    println!("B");
                    break
                }
                turn = sb[b];
                b += 1;
            },
            'c' => {
                if c == sc.len() {
                    println!("C");
                    break
                }
                turn = sc[c];
                c += 1;
            },
            _ => {},
        }
    }
}