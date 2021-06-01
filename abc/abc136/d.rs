use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        c: Chars
    }

    let mut map = Vec::new();

    let mut count = 1;
    let mut ch = 'R';
    for i in 1..c.len() {
        if c[i] == ch {
            count += 1;
        } else {
            map.push(count);
            count = 1;
            ch = c[i];
        }
    }
    map.push(count);
    // println!("{:?}", map);
    
    for i in 0..map.len() / 2 {
        let l = map[2 * i + 1];
        let r = map[2 * i];

        for n in 0..r - 1 {
            print!("0 ");
        }
        print!("{} ", (r + 1) / 2 + l / 2);
        print!("{} ", r / 2 + (l + 1) / 2);
        for n in 0..l - 1 {
            print!("0 ");
        }
    }
}