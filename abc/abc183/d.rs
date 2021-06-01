use proconio::input;

fn main() {
    input! {
        n: usize,
        w: i64,
        stp: [(usize, usize, i64); n],
    }

    let mut water = vec![0; 200_001];
    for (s, t, p) in stp {
        water[s] += p;
        water[t] -= p;
    }
    let mut sum = 0;
    for j in water {
        sum += j;
        if sum > w {
            println!("No");
            return
        }
    }
    println!("Yes");
}
