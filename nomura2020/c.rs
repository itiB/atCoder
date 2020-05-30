use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [f64; n],
        a_last: f64
    }

    let mut sum = 0;
    let mut pred = 0.5;
    let mut flag = 0.0;
    let mut max = a.iter

    for stage in a {
        pred = (pred * 2.0) - stage;
        if pred <= 0.0 {
            println!("-1");
            return;
        }
        if pred >= max {
            pred = max;
            flag = stage;
        }
        sum += (pred + stage) as i64;
        // println!("stage:{}, pred: {}, sum: {}", stage, pred, sum);
    }

    sum += flag as i64;

    if a_last > pred * 2.0 {
        println!("-1");
    } else {
        println!("{}", sum + a_last as i64);
    }
}