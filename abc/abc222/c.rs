use proconio::input;
use proconio::marker::Chars;
use std::collections::{BTreeMap, HashMap};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Chars; 2*n]
    }

    let mut ans = vec![vec![None; m]; 2*n];

    let mut past = Vec::new(); // その前の勝利数
    for i in 0..2*n {
        ans[i][0] = Some(0);
        past.push((0, i));
    }
    for i in 1..m {
        past.sort();
        let mut tmp = Vec::new();
        for j in 0..n {
            // 1位の人: past[j*2].1
            // 得点: past[j*2].0
            // 2位の人: past[j*2+1].1
            // 得点: past[j*2+1].0
            // println!("{}: {} x {}",j,a[past[j*2].1][i-1], a[past[j*2+1].1][i-1]);
            if ans[past[j*2  ].1][i] == None {
                match a[past[j*2].1][i-1] {
                    'G' => {
                        match a[past[j*2+1].1][i-1] {
                            'G' => {
                                // あいこ
                                ans[past[j*2  ].1][i] = Some(ans[past[j*2  ].0][i-1].unwrap());
                                ans[past[j*2+1].1][i] = Some(ans[past[j*2+1].0][i-1].unwrap());
                                tmp.push((ans[past[j*2].1][i].unwrap(), past[j*2].1));
                                tmp.push((ans[past[j*2+1].1][i].unwrap(), past[j*2+1].1));
                            },
                            'C' => {
                                // かち
                                ans[past[j*2  ].1][i] = Some(ans[past[j*2  ].0][i-1].unwrap()+1);
                                ans[past[j*2+1].1][i] = Some(ans[past[j*2+1].0][i-1].unwrap());
                                tmp.push((ans[past[j*2].1][i].unwrap(), past[j*2].1));
                                tmp.push((ans[past[j*2+1].1][i].unwrap(), past[j*2+1].1));
                            },
                            'P' => {
                                // まけ
                                ans[past[j*2  ].1][i] = Some(ans[past[j*2  ].0][i-1].unwrap());
                                ans[past[j*2+1].1][i] = Some(ans[past[j*2+1].0][i-1].unwrap()+1);
                                tmp.push((ans[past[j*2].1][i].unwrap(), past[j*2].1));
                                tmp.push((ans[past[j*2+1].1][i].unwrap(), past[j*2+1].1));
                            },
                            _ => {},
                        }
                    },
                    'C' => {
                        match a[past[j*2+1].1][i-1] {
                            'G' => {
                                // まけ
                                ans[past[j*2  ].1][i] = Some(ans[past[j*2  ].0][i-1].unwrap());
                                ans[past[j*2+1].1][i] = Some(ans[past[j*2+1].0][i-1].unwrap()+1);
                                tmp.push((ans[past[j*2].1][i].unwrap(), past[j*2].1));
                                tmp.push((ans[past[j*2+1].1][i].unwrap(), past[j*2+1].1));},
                            'C' => {
                                // あいこ
                                ans[past[j*2  ].1][i] = Some(ans[past[j*2  ].0][i-1].unwrap());
                                ans[past[j*2+1].1][i] = Some(ans[past[j*2+1].0][i-1].unwrap());
                                tmp.push((ans[past[j*2].1][i].unwrap(), past[j*2].1));
                                tmp.push((ans[past[j*2+1].1][i].unwrap(), past[j*2+1].1));},
                            'P' => {
                                // かち
                                ans[past[j*2  ].1][i] = Some(ans[past[j*2  ].0][i-1].unwrap()+1);
                                ans[past[j*2+1].1][i] = Some(ans[past[j*2+1].0][i-1].unwrap());
                                tmp.push((ans[past[j*2].1][i].unwrap(), past[j*2].1));
                                tmp.push((ans[past[j*2+1].1][i].unwrap(), past[j*2+1].1));
                            },
                            _ => {},
                        }
                    },
                    'P' => {
                        match a[past[j*2+1].1][i-1] {
                            'G' => {
                                // かち
                                ans[past[j*2  ].1][i] = Some(ans[past[j*2  ].0][i-1].unwrap()+1);
                                ans[past[j*2+1].1][i] = Some(ans[past[j*2+1].0][i-1].unwrap());
                                tmp.push((ans[past[j*2].1][i].unwrap(), past[j*2].1));
                                tmp.push((ans[past[j*2+1].1][i].unwrap(), past[j*2+1].1));},
                            'C' => {
                                // まけ
                                ans[past[j*2  ].1][i] = Some(ans[past[j*2  ].0][i-1].unwrap());
                                ans[past[j*2+1].1][i] = Some(ans[past[j*2+1].0][i-1].unwrap() + 1);
                                tmp.push((ans[past[j*2].1][i].unwrap(), past[j*2].1));
                                tmp.push((ans[past[j*2+1].1][i].unwrap(), past[j*2+1].1));},
                            'P' => {
                                // あいこ
                                ans[past[j*2  ].1][i] = Some(ans[past[j*2  ].0][i-1].unwrap());
                                ans[past[j*2+1].1][i] = Some(ans[past[j*2+1].0][i-1].unwrap());
                                tmp.push((ans[past[j*2].1][i].unwrap(), past[j*2].1));
                                tmp.push((ans[past[j*2+1].1][i].unwrap(), past[j*2+1].1));
                            },
                            _ => {},
                        }
                    },
                    _ => {}
                }

                // for a in &ans {
                //     println!("{:?}", a);
                // }
            }
        }
        past = tmp.clone();
    }
    // past.sort_by_key(|&x| Reverse(x));
    // past.sort_by_key(|a| a.1);
    past.sort();
    println!("{:?}", past);
    past.sort_by_key(|&t| ((t.1 as isize)));
    println!("{:?}", past);
    // past.into_iter().for_each(|t| println!("{}", t.0+1));

}