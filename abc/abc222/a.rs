use proconio::input;

fn main() {
    input! {
        n: usize
    }

    if n > 999 {
        println!("{}", n);
    } else if n > 99 {
        print!("0");
        println!("{}", n);
    } else if n > 9 {
        print!("00");
        println!("{}", n);
    } else {
        print!("000");
        println!("{}", n);
    }
}