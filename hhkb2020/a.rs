use proconio::input;

fn main() {
    input! {
        s: char,
        t: char,
    }

    if s == 'Y' {
        println!("{}", 
        match t {
            'a' => 'A',
            'b' => 'B',
            'c' => 'C',
            _ => unreachable!(),
        });
    } else {
        println!("{}", t);
    }
}