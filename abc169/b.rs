use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i128; n],
    }
    let find = a.iter().find(|x| x == &&0);
    match find {
        Some(_) => println!("0"),
        None => get_num(a),
    }
}

fn get_num(a: Vec::<i128>) {
    let mut ans = 1;
    let bound = 10i128.pow(18u32);
    for num in a {
        ans *= num;
        if ans > bound {
            println!("-1");
            return;
        }
    }
    println!("{}", ans);
}