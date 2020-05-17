use proconio::input;
use std::f64::consts::PI;

fn main() {
    input! {
        a: f64,
        b: f64,
        h: f64,
        m: f64,
    }

    let h_theta = 2.0 * PI * (h + m / 60.0) / 12.0;
    let m_theta = 2.0 * PI * m / 60.0;

    let x = length(a * h_theta.cos(), b * m_theta.cos());
    let y = length(a * h_theta.sin(), b * m_theta.sin());

    println!("{}", (x * x + y * y).sqrt());
}

fn length(a: f64, b: f64) -> f64 {
    if a < b {
        return b - a
    } else {
        return a - b
    }
}
