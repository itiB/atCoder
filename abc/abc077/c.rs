use proconio::input;

fn main() {
    input! {
        n: i32,
        mut a: [i32; n],
        mut b: [i32; n],
        mut c: [i32; n],
    }

    a.push(std::i32::MAX);
    a.push(std::i32::MIN);
    a.sort();
    c.push(std::i32::MAX);
    c.push(std::i32::MIN);
    c.sort();

    // println!("{:?}", c);

    let mut ans = 0;
    for i in b {
        // println!("{}", i);
        let aa = binary_search(&a, i, terms) - 1;
        let cc = c.len() - 1 - binary_search(&c, i, terms2);
        ans += aa * cc;
        // println!("c {}, len {}", cc, c.len());
    }
    println!("{}", ans);
}

fn terms(num: i32, value: i32) -> bool {
    // 条件をここに作る
    // terms(right) = true になるように
    if num >= value {
        true
    } else {
        false
    }
}

fn terms2(num: i32, value: i32) -> bool {
    // 条件をここに作る
    // terms(right) = true になるように
    if num > value {
        true
    } else {
        false
    }
}

fn binary_search<F: Fn(i32, i32) -> bool>(vec: &[i32], value: i32, terms: F) -> usize {
    // 入力はバグらないように前後にINFをいれたソート済みベクタ
    let mut l = 0;
    let mut r = vec.len() - 1;

    while r > l + 1 {
        let m = l + (r - l) / 2;
        if terms(vec[m], value) {
           r = m;
        } else {
            l = m;
        }
    }
    r
}
