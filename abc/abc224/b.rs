use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h]
    }

    for i1 in 1..h { // 50
        for i2 in i1+1..=h { // 50
            for j1 in 1..w { // 50
                for j2 in j1..=w {
                    if a[i1-1][j1-1] + a[i2-1][j2-1] > a[i2-1][j1-1] + a[i1-1][j2-1] {
                        println!("No");
                        return
                    }
                }
            }
        }
    }
    println!("Yes");
}