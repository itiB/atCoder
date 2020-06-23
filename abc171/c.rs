use proconio::input;

fn main() {
    input! {
        mut n: i128,
    }
    let mut name = Vec::new();
    loop {
        n -= 1;
        name.push(n % 26);
        n /= 26;
        if n == 0 { break }
    }
    for c in (0..name.len()).rev() {
        print!("{}", decoder(name[c]))
    }
    println!("");
}

fn decoder(input: i128) -> char {
    let a = 'a' as u8;
    (a + input as u8) as char
}