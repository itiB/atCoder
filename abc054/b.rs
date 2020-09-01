e proconio::input;
    use proconio::marker::Chars;
     
    fn main() {
        input! {
            n: usize,
            m: usize,
            a: [Chars; n],
            b: [Chars; m],
        }
     
        for ax in 0..n - m + 1 {
            for ay in 0..n - m + 1 {
                'check: for bx in 0..m {
                    for by in 0..m {
                        if a[ax + bx][ay + by] != b[bx][by] {
                            break 'check;
                        }
                    }
                    if bx == m - 1 {
                        println!("Yes");
                        return;
                    }
                }
            }
        }
        println!("No");
    }
