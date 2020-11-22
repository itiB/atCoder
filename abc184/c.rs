use proconio::input;

fn main() {
    input! {
        r1: i128,
        c1: i128,
        r2: i128,
        c2: i128
    }

    if r1 == r2 && c1 == c2 {
        println!("0");
    } else if r1 + c1 == r2 + c2 ||
        r1 - c1 == r2 - c2 ||
        (c1 - c2).abs() + (r1 - r2).abs() < 4 {
        println!("1");
    } else if
        (r1 + 1) + c1 == r2 + c2 ||
        (r1 + 2) + c1 == r2 + c2 ||
        (r1 + 3) + c1 == r2 + c2 ||
        (r1 - 1) + c1 == r2 + c2 ||
        (r1 - 2) + c1 == r2 + c2 ||
        (r1 - 3) + c1 == r2 + c2 ||
        (r1 + 1) - c1 == r2 - c2 ||
        (r1 + 2) - c1 == r2 - c2 ||
        (r1 + 3) - c1 == r2 - c2 ||
        (r1 - 1) - c1 == r2 - c2 ||
        (r1 - 2) - c1 == r2 - c2 ||
        (r1 - 3) - c1 == r2 - c2 ||
        r1 + r2 + c1 + c2 == 0 ||
        {
            (c1 * c2 * c2 + c1 * r2 * r2) % (r1 * r2 + c1 * c2) == 0 &&
            (r1 * c2 * c2 + r1 * r2 * r2) % (r1 * r2 + c1 * c2) == 0 
        } ||
        (c1 - c2).abs() + (r1 - r2).abs() < 7 {
            println!("2");
    } else {
        println!("3");
    }
}