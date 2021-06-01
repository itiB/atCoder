use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        xy: [(usize, usize); m]
    }

    let mut map = [[false; 20]; 20];
    for (x, y) in xy {
        map[x][y] = true;
    }

    let mut j = 0;
    let mut v = Vec::new();
    v.push(n);
    while j < 2 * n {
        j += 1;
        let mut vv = Vec::new();
        println!("{:?}", v);
        for target in &v {
            if map[j + 1][*target] == false {
                println!("1");
                vv.push(*target);
            }
            if *target < 2 * n { if map[j + 1][*target + 1] == true {
                println!("2");
                vv.push(*target + 1);
            }}
            if *target >= 1 { if map[j + 1][*target - 1] == true {
                println!("3");
                vv.push(*target - 1);
            }}
        }
        v = vv;
    }
}