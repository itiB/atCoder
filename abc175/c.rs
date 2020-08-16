use proconio::input;

fn main() {
    input! {
        mut x: i128,
        mut k: usize,
        d: i128,
    }

    let mut last = 0;
    let mut count = x/d;

    if x > count * d {
        x -= count *d;
    } else {
        x += 
    }

    for i in 0..k - count {
        if (x + d).abs() > (x - d).abs() {
            x -= d;
            if last == -1 {
                if (k - i + 1) % 2 != 0 {
                    x += d;
                }
                break; 
            }
            last = 1;
        } else {
            x += d;
            if last == 1 {
                if (k - i + 1) % 2 != 0 {
                    x -= d;
                }
                break;
            }
            last = -1;
        }
    }
    println!("{}", x.abs());
}