use proconio::input;

fn main() {
    input! {
        d: i32,
        t: i32,
        s: i32,
    }

    if d <= t * s { 
        println!("Yes");
    } else {
        println!("No");
    }
}