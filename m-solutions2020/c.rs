use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [u128; n],
    }

    let mut vec = Vec::new();

    let mut point = 1;
    for num in 0..k {
        point *= a[num];
    }
    vec.push(point);

    for num in k..n {
        vec.push(vec[vec.len() - 1] / a[num - k] * a[num]);
    }

    let mut past_point = vec[0];
    for num in 1..=n - k{
        if past_point < vec[num] {
            println!("Yes");
        } else {
            println!("No");
        }
        past_point = vec[num];
    }
}