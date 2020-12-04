use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
        c: f64
    }

    let mut left = 0.0;
    let mut right = 300.0;

    for _ in 0..100 {
        let t = (left + right) / 2.0;
        if a * t + b * (c * t * std::f64::consts::PI).sin() > 100.000001 {
            right = t;
        } else {
            left = t;
        }
    }
    println!("{}", left);
}

