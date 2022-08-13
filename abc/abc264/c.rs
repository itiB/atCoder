use proconio::input;

fn main() {
    input! {
        h1: usize,
        w1: usize,
        a: [[usize; w1]; h1],
        h2: usize,
        w2: usize,
        b: [[usize; w2]; h2],
    }

    for sw in 0..1 << w1 {
        let mut use_w = Vec::new();
        for j in 0..w1 {
            if sw >> j & 1 == 1 {
                use_w.push(j)
            }
        }
        if use_w.len() == w2 {
            // println!("{:?}", use_w);
            for sh in 0..1 << h1 {
                let mut use_h = Vec::new();
                for k in 0..h1 {
                    if sh >> k & 1 == 1 {
                        use_h.push(k)
                    }
                }
                if use_h.len() == h2 {
                    // println!("{:?}", use_h);
                    let mut flag = true;
                    for hi in 0..h2 {
                        for wi in 0..w2 {
                            if b[hi][wi] != a[use_h[hi]][use_w[wi]] {
                                flag = false;
                            }
                        }
                    }
                    if flag {
                        println!("Yes");
                        return;
                    }
                }
            }
        }
    }
    println!("No");
}
