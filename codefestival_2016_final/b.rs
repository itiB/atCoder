use proconio::input;

fn main() {
    input! {
        n: i128,
    }

    let mut sum = 0;

    for i in 1..=n {
        sum += i;
        if sum >= n {
            let diff = sum - n;

            for j in 1..=i {
                if j != diff {
                    println!("{}", j);
                }
            }
            break;
        }
    }
}