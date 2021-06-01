use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let week = ["SAT", "FRI", "THU", "WED", "TUE", "MON", "SUN"];
    for i in 0..7 {
        if week[i] == s {
            println!("{}", i + 1);
        }
    }
}