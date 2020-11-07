use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        sc: [(usize, usize); m],
    }

    let mut num: Vec<Option<usize>> = vec![None; n];

    if m == 0 && n == 1 {
        println!("0");
        return
    }

    for (s, c) in sc {
        if let Some(m) = num[s - 1] {
            if m != c {
                println!("-1");
                return
            }
        }
        num[s - 1] = Some(c);
    }
    if let Some(m) = num[0] {
        if n != 1 && m == 0 {
            println!("-1");
            return
        }
    } else {
        num[0] = Some(1);
    }

    let mut count = 1;
    let mut ans = 0;

    for i in (0..n).rev() {
        if let Some(n) = num[i] {
            ans += n * count;
        }
        count *= 10;
    }
    println!("{}", ans);
}