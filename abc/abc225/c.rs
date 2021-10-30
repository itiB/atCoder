use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        b: [[usize; m]; n]
    }

    let mut past = (b[0][0] - 1) % 7;
    for j in 1..m {
        if b[0][j] != b[0][j-1]+1 {
            println!("No");
            return
        }
        let now = (b[0][j] - 1) % 7;
        if past >= now {
            println!("No");
            return
        }
        past = now;
    }
    for i in 1..n {
        if b[i][0] != b[i-1][0] + 7 {
            println!("No");
            return
        }
        past = (b[i][0] - 1) % 7 ;
        for j in 1..m {
            if b[i][j] != b[i][j-1] + 1 {
                println!("No");
                return
            }
            let now = (b[i][j] - 1) % 7;
            if past >= now {
                println!("No");
                return
            }
            past = now;
        }
    }
    println!("Yes");
}
