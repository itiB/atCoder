use proconio::input;

fn main() {
    input! {
        c: char,
    }

    match c {
        'a' ..= 'z' => println!("a"),
        'A' ..= 'Z' => println!("A"),
        _ => println!("Err"),
    }
}
