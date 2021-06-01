use proconio::input;

fn main() {
    input! {
        k: i32,
        s: i32,
    }

    let mut ans = 0;

    for x in 0..=k {
        for y in 0..=k {
            let z = s - x - y;
            if 0 <= z && z <= k {
                ans += 1;
            }
            // for z in 0..=k {
            //     if x + y + z == s {
            //         ans += 1;
            //     }
            // }
        }
    }
    println!("{}", ans);
}