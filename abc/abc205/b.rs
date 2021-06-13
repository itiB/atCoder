use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut v = vec![false; n + 1];
    v[0] = true;
    for i in a {
        if v[i] == true {
            println!("No");
            return;
        } else {
            v[i] = true;
        }
    }
    println!("{}", if v.iter().filter(|x| **x == false).count() > 0 { "No" } else { "Yes" });
}