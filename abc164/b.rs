use proconio::input;

fn main() {
    input! {
        mut a: i32,
        b: i32,
        mut c: i32,
        d: i32,
    }
    loop {
        if a <= 0 {
            if c <= 0 {
                println!("Yes");
            } else {
                println!("No");
            }
            break;
        }
        if c <= 0 {
            println!("Yes");
            break;
        }
        a = a - d;
        c = c - b;
    }
}