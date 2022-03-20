use proconio::input;
fn main() {
    input! {
        s1: char,
        s2: char,
        s3: char,
        t1: char,
        t2: char,
        t3: char
    }

    let list = vec![(s1, s2, s3), (s2, s3, s1), (s3, s1, s2)];
    for (a, b, c) in list {
        if t1 == a && t2 == b && t3 ==c {
            println!("Yes");
            return
        }
    }
    println!("No");
}