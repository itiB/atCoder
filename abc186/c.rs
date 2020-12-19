use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut ans = 0;
    for i in 1..=n {
        if !(i.to_string().contains('7') || format!("{:o}", i).to_string().contains('7')){
            ans += 1;
        }
    }
    println!("{}", ans)
}
