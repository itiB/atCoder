use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        d: i64,
    }

    println!("{}", 
    if c * d < b {
         -1
        } else if c * d == b {
            if a == 0 { 1 } else { -1 }
        } else {
             (a + c * d - b - 1) / (c * d - b)
        }
    );
}