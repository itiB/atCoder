use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [usize; n]
    }

    let mut ans = 0;
    for i in s {
        let mut f = false;
        for a in 1..=334 {
            for b in 1..=334 {
                if 4 * a * b + 3 * a + 3 * b == i {
                    f = true;
                    break;
                }
            }
            if f == true {
                break
            }
        }
        if f == false {
            ans += 1;
        }
    }
    println!("{}", ans);
}