use std::io;

fn main() {
    real_main();
}

fn real_main() -> i32 {
    let num = read::<i32>();

    if num > 81 {
        println!("No");
        return 0
    } else if num < 1 {
        println!("No");
        return 0
    } else if num == 1 {
        println!("Yes");
        return 0
    }

    for i in (1..10).rev(){
        if num % i == 0{
            let num2 = num / i;
            // println!("{} / {} = {}", num, i, num2);
            for j in (1..i + 1).rev(){
                // println!("    {} / {} = {}", num2, j, num2 /j );
                if num2 / j == 1{
                    println!("Yes");
                    return 0
                }
            }
        }
    }
    println!("No");
    return 0
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}