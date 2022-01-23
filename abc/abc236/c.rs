use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [String;n],
        t: [String;m]
    }

    let mut kyuko = 0;
    for station in s {
        if station == t[kyuko] {
            println!("Yes");
            kyuko += 1;
        } else {
            println!("No");
        }
    }
}