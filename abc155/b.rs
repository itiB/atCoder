use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut flag = true;
    for num in a {
        if num % 2 == 0 && num % 3 != 0 && num % 5 != 0 {
                flag = false;
        }
    }
    if flag {
        println!("APPROVED");
    } else {
        println!("DENIED");
    }
}