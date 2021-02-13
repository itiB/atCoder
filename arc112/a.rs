use proconio::input;

fn main() {
    input! {
        n: i64,
        p: [i64; n - 1]
    }

    let mut nodes = Vec::<Node>::new();
    for i in 0..n {
        nodes.push( Node{
            id: i,
            parent: None,
            depth: 0,
            children: Vec::new(),
        });
    }
    let mut tree = Tree { nodes: nodes };


    // for _ in 0..n {
    //     let childs = if p.len() < 3 {
    //         [].to_vec()
    //     } else {
    //         p[2..].to_vec()
    //     };
    //     tree.update_node(p[0], childs);
    // }

    let mut id_count = 1;
    for i in p {
        tree.update_node(i, tree[i].children.push(id_count));
    }
    //     nodes.push( Node{
    //         id: count,
    //         parent: Some(i),
    //         depth: 0,
    //         children: Vec::new(),
    //     });
    //     count += 1;
    // }
    // let mut tree = Tree { nodes: nodes };

    // let root = tree.find_root();
    // tree.fill_depth(root, 0);

    // for id_node in 0..n {
    //     let node = &tree.nodes[id_node as usize];
    //     println!("{}", node);
    // }
}


use std::fmt;
type NodeNum = i64;

#[derive(Debug)]
struct Node {
    id: NodeNum,
    parent: Option<NodeNum>,
    depth: usize,
    children: Vec<NodeNum>,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let node_type = if self.depth == 0 {
            "root"
        } else if self.children.len() == 0 {
            "leaf"
        } else {
            "internal node"
        };

        let p = match self.parent {
            Some(val) => val,
            None => -1,
        };

        write!(f, "node {}: parent = {:?}, depth = {}, {}, {:?}", self.id, p, self.depth, node_type, self.children)
    }
}

#[derive(Debug)]
struct Tree {
    nodes: Vec<Node>,
}

impl Tree {
    // fn count_nodes(&self) -> usize {
    //     self.nodes.len()
    // }

    fn update_node(&mut self, id: NodeNum, childs: Vec<NodeNum>) {
        if childs.len() == 0 { return; }

        let copy_childs = childs.clone();

        self.nodes[id as usize].children = childs;

        for child in copy_childs {
            self.nodes[child as usize].parent = Some(id);
        }
    }

    fn find_root(&self) -> NodeNum {
        let mut current: NodeNum = 0;
        loop {
            match self.nodes[current as usize].parent {
                Some(id) => current = id,
                None => break,
            }
        }
        current
    }

    fn fill_depth(&mut self, root_id: NodeNum, depth: usize) {
        self.nodes[root_id as usize].depth = depth;

        let copy_childs = self.nodes[root_id as usize].children.clone();

        for child in copy_childs {
            self.fill_depth(child, depth + 1);
        }
    }
}
