use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut i = 1;
    while 5_usize.pow(i) < n {
        let mut num = n - 5_usize.pow(i);
        if checker(num) {
            let mut count = 0;
            while num != 1 {
                num /= 3;
                count += 1;
            }
            println!("{} {}", count, i);
            return
        }
        i += 1;
    }
    println!("-1");
}

// 3の累乗か判定
fn checker(n: usize) -> bool {
    if n % 3 == 0{
        if n / 3 != 1 {
            checker(n / 3)
        } else {
            true
        }
    } else {
        false
    }
}
