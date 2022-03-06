use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        q: usize,
        // tk: [(usize, usize); q]
    }
    let si = s.iter().map(|&x| x as usize - 'A' as usize).collect::<Vec<_>>();
    let map = vec!['A', 'B', 'C'];

    for _ in 0..q {
        input! {
            t: usize,
            mut k: usize,
        }
        k -= 1;

        // 二進数で何番目か考える
        // 左に進んだ回数と右に進んだ回数を考えると...
        // 1の個数=右に行く回数
        let mut start_word = 0;
        if t <= 60 {
            let p = 1 << t;
            start_word = k / p;
            k %= p;
        }

        // 右に行く回数
        let r = (k as u64).count_ones() as usize;
        let l = t - r;
        let x = l + r * 2 + si[start_word as usize];
        println!("{}", map[x % 3]);
    }
}