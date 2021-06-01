use proconio::input;

fn main() {
    input! {
        n: i32,
        a: [i32; n - 1],
    }

    let mut list = vec![0; n as usize];

    for jyoshi in a {
        list[jyoshi as usize - 1] += 1;
    }
    for i in &list {
        println!("{}", i);
    }
}
