use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut i = 1;
    let mut stack = Vec::new();
    loop {
        if i * i > n {
            break;
        }
        if n % i ==0 {
            if n / i != i {
                stack.push(n / i);
            }
            println!("{}", i);
        }
        i += 1;
    }
    for s in stack.iter().rev() {
        println!("{}", s);
    }

}
