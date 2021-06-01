use proconio::input;

fn main() {
    input! {
        x: i64
    }

    let mut mon: i64 = 100;
    let mut cou: i32 = 0;

    while mon < x {
        mon = mon + mon / 100;
        cou = cou + 1;
    }
    println!("{}", cou);
}