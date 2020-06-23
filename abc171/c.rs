use proconio::input;

fn main() {
    input! {
        mut n: i128,
    }
    let mut name = Vec::new();
    loop {
        let mut number = n % 26;
        if number == 0 {
            number = 26;
            n = n / 26 - 1;
        } else {
            n = n / 26;
        }
        name.push(number);

        if n <= 0 { break }
    }
    for c in (0..name.len()).rev() {
        print!("{}", decoder(name[c]))
    }
    println!("");
}

fn decoder(input: i128) -> char {
    let a = 'a' as u8;
    (a - 1 + input as u8) as char
}