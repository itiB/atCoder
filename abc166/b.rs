use proconio::input;

fn main() {
    input! {
        n: i32,
        k: i32,
    }
    let mut res: i32 = 0;

    let mut sunuke: Vec::<i32> = Vec::new();

    for _i in 0..k {
        input! {
            mut i: i32,
            mut s: [i32; i],
        }
        sunuke.append(&mut s);
    }
    sunuke.sort();
    sunuke.dedup();

    for _i in 1..=n {
        if !sunuke.contains(&(_i)) {
            res += 1;
        }
    }
    println!("{}", res);
}
