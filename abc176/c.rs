use proconio::input;
// use proconio::marker::{Bytes, Chars};

fn main() {
    input! {
        n: i128,
        a: [i128; n],
    }

    let mut height = a[0];
    let mut sum = 0;

    for person in a {
        if height > person {
            // print!("{} ", height - person);
            sum += height - person;
        } else {
            // print!("0 ");
            height = person;
        }
    }
    println!("{}", sum);
}