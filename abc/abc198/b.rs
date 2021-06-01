use proconio::input;

fn main() {
    input! {
        mut n: usize,
    }

    if n != 0 {
        while n % 10 == 0 {
            n /= 10;
        }
        let str_n: Vec<char> = n.to_string().chars().collect();
        for i in 0..str_n.len()/2 {
            if str_n[i] != str_n[str_n.len() - i - 1] {
                println!("No");
                return
            }
        }
    }
    println!("Yes");
}