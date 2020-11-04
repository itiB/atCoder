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
    b.push(std::i32::MAX);
    b.push(std::i32::MIN);
    b.sort();

    let mut ans = 0;
    for i in c {
        // m以降は指定値より大きくなるので見ない
        let m = binary_search(&b, i);
        for j in 1..m {
            // binary_searchで数列の中で指定した値を越えるところがわかる
            // a[a.len() - 1] = INF が含まれているため消す
            ans += binary_search(&a, b[j]) - 1;
        }
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

fn binary_search(vec: &[i32], value: i32) -> usize {
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
