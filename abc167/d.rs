use proconio::input;

fn main() {
    input! {
        n: usize,
        mut k: i64,
        a: [i64; n],
    }

    let mut looped = vec![false; n];
    looped[0] = true;
    let mut go: i64 = 1;
    let mut count = 0;
    let mut loopstart = 0;
    let mut past = k;
    loop {
        go = a[(go - 1) as usize];
        count += 1;
        if looped[(go - 1) as usize] == true {
            // ここでループに入ったことを検知
            if go != loopstart {
                loopstart = go;
            }
            break
        }
        looped[(go - 1) as usize] = true;
    }

    go = 1;
    loop {
        if go == loopstart {
            break;
        }
        go = a[(go - 1) as usize];
        k -= 1;
        count -= 1;
    }
    if k < 0{
        // ループにすらジャンプ回数が足りないときの処理
        go = 1;
        for _ in 0..past {
            go = a[(go - 1) as usize];
        }
        println!("{}", go);
    } else {
        go = loopstart;
        for _ in 0..k % count {
            go = a[(go - 1) as usize];
        }
        println!("{}", go);
    }
}