use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize; n],
        x: [usize; q]
    }

    a.push(0);
    a.push(1_000_000_001);
    a.sort();
    for xx in x {
        let ans = binary_search(&a, xx);
        println!("{}", n - (ans - 1));
    }
}


fn terms(num: usize, value: usize) -> bool {
    // 条件をここに作る
    // terms(right) = true になるように
    if num >= value {
        true
    } else {
        false
    }
}

fn binary_search(vec: &[usize], value: usize) -> usize {
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
