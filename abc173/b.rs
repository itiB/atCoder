use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut c0 = 0;
    let mut c1 = 0;
    let mut c2 = 0;
    let mut c3 = 0;

    for req in s {
        match &*req {
            "AC" => c0 += 1,
            "WA" => c1 += 1,
            "TLE" => c2 += 1,
            "RE" => c3 += 1,
            _ => {},
        }
    }
    println!("AC x {}", c0);
    println!("WA x {}", c1);
    println!("TLE x {}", c2);
    println!("RE x {}", c3);
}