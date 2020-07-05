use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        hw: [String; h],
    }

    let mut hc = vec![0; h];
    let mut wc = vec![0; w];
    let mut all = 0;
    for l in 0..h {
        // println!("{:?}", hw[l]);
        let chars: Vec<char> = hw[l].chars().collect();

        let mut sum = 0;
        for c in 0..w {
            if chars[c] == '#' {
                wc[c] += 1;
                sum += 1;
            }
        }
        hc[l] = sum;
        all += sum;
    }

    let mut ans = 0;

    // println!("{:?}", wc);
    for s in 0..1 << h {
        let mut all_black = all;
        let mut this_line = wc.to_vec();
        // println!("{:b}", s);
        for i in 0..h {
            // もしi行目を消すのならば
            if s >> i & 1 == 1 {
                all_black -= hc[i];
                let chars2: Vec<char> = hw[i].chars().collect();
                // println!("rm: {:?}", hw[i]);
                for c in 0..w {
                    if chars2[c] == '#' {
                        this_line[c] -= 1;
                    }
                }
            }
        }
        // println!("wc2: {:?}, {}", this_line, all_black);
        if all_black > k {
            let mut need = all_black - k;

            for sw in 0..1 << w {
                let mut w_sum = 0;
                for j in 0..w {
                    if sw >> j & 1 == 1 {
                        w_sum += this_line[j];
                    }
                }
                if w_sum == need {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);
}