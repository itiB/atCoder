use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [f64; n],
        a_last: f64,
    }

    let mut sum = 0;
    let mut pred = 0.5;
    let mut max = a.iter().fold(0.0, |s, i| s + i) + a_last;

    for stage in a {
        max -= stage;
        pred = (pred * 2.0) - stage;
        if pred >= max {
            pred = max;
        }
        sum += (pred + stage) as i64;
        // println!("stage:{}, pred: {}, sum: {}", stage, pred, sum);
    }

    if a_last > pred * 2.0 {
        println!("-1");
    } else {
        println!("{}", sum + a_last as i64);
    }
}