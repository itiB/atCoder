use proconio::input;
use std::cmp::{min, max};

fn main() {
    input! {
        l: usize,
        q: usize,
        cx: [(usize, usize); q]
    }
    let mut s = Node::new(l);
    for (c, x) in cx {
        match c {
            1 => {
                s.insert(x);
            },
            2 => {
                let (minimum, maximum) = s.search_side(x, 0, l);
                println!("{}", maximum - minimum);
            },
            _ => {}
        }
    }
}

// ref https://totechite.hatenablog.com/entry/2019/03/06/010020
use std::boxed::Box;

#[derive(Debug)]
pub struct Node
{
    data: usize,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node
{
    pub fn new(num: usize) -> Self
    {
        Self {
            data: num,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, num: usize)
    {
        if &self.data > &num {                  //Left Node
            if let Some(left_node) = &mut self.left {
                left_node.insert(num);
            } else {
                let node = Self::new(num);
                self.left = Some(Box::new(node));
            }
        } else if &self.data < &num {           //Right Node
            if let Some(right_node) = &mut self.right {
                right_node.insert(num);
            } else {
                let node = Self::new(num);
                self.right = Some(Box::new(node));
            }
        } else {}                               //既にある場合なにもしない
    }

    pub fn search(&self, num: usize) -> bool
    {
        if &self.data > &num {                  //Left Node
            if let Some(left_node) = &self.left {
                left_node.search(num)
            } else { false }
        } else if &self.data < &num {           //Right Node
            if let Some(right_node) = &self.right {
                right_node.search(num)
            } else { false }
        } else { true }
    }

    pub fn search_side(&self, num: usize, mut minimum: usize, mut maximum: usize) -> (usize, usize) {
        if &self.data > &num {                  //Left Node
            maximum = min(maximum, self.data);
            if let Some(left_node) = &self.left {
               left_node.search_side(num, minimum, maximum)
            } else { (minimum,maximum) }
        } else if &self.data < &num {           //Right Node
            minimum = max(minimum, self.data);
            if let Some(right_node) = &self.right {
                right_node.search_side(num, minimum, maximum)
            } else { (minimum, maximum) }
        } else { (minimum, maximum) }
    }
}