extern crate boo;

#[allow(unused_imports)]
use boo::{Forest, Node, add, multiply};
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let f = &mut Forest::new();

    let x = f.to_node_idx(Node(0, 1, 0));
    let y = f.to_node_idx(Node(1, 1, 0));

    let x_add_y = add(f, x, y);
    let y_add_x = add(f, y, x);

    let mut file = File::create("foo.txt").unwrap();
    writeln!(file, "digraph {{").unwrap();
    f.write_graph(&mut file, x_add_y).unwrap();
    writeln!(file, "}}").unwrap();

    writeln!(file, "digraph {{").unwrap();
    f.write_graph(&mut file, y_add_x).unwrap();
    writeln!(file, "}}").unwrap();
}
