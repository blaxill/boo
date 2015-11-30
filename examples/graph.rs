extern crate boo;

use boo::{Forest, Node, add, multiply};
use std::io::prelude::*;
use std::io::{self, SeekFrom};
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
}
