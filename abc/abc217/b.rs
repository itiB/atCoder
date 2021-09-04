use proconio::input;

fn main() {
    input! {
        s: [String; 3]
    }

    let mut m = vec![false; 4];
    for ss in s {
        match &*ss {
            "ABC" => m[0] = true,
            "ARC" => m[1] = true,
            "AGC" => m[2] = true,
            "AHC" => m[3] = true,
            _ => {}
        }
    }
    if !m[0] { println!("ABC"); }
    if !m[1] { println!("ARC"); }
    if !m[2] { println!("AGC"); }
    if !m[3] { println!("AHC"); }
}