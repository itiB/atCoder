use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut t: Chars
    }

    let mut pred = 'D';
    for c in 0..t.len() {
        if t[c] == 'P' {
            print!("P");
            pred = 'P'
        }
        if t[c] == 'D' {
            print!("D");
            pred = 'D'
        }
        if t[c] == '?' {
            if pred == 'P' {
                print!("D");
                pred = 'D';
            } else {
                if c < t.len() - 1 {
                    if t[c + 1] == 'D' || t[c + 1] == '?' {
                        print!("P");
                        pred = 'P';
                    } else {
                        print!("D");
                        pred = 'D';
                    }
                } else {
                    print!("D");
                    pred = 'D';
                }
            }
        }
    }
    print!("\n");
}