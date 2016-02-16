extern crate boo;

use boo::{Forest, Node, add};
use std::io::stdout;

fn main() {
    let f = &mut Forest::new();

    let x = f.to_node_idx(Node(0, 1, 0));
    let y = f.to_node_idx(Node(1, 1, 0));

    let x_add_y = add(f, x, y);

    println!("digraph {{");
    f.write_graph(&mut stdout(), x_add_y).unwrap();
    println!("}}");
}
