use forest::{Forest, Node};
use grobner::normal_form;

mod forest;
mod grobner;

fn print_v(v: Vec<usize>) {
    for e in v.iter() {
        print!("{}, ", e);
    }
    println!("");
}

fn main(){
    let mut f = Forest::new();
    let x = f.get_node_id(Node::Variable(0, 1, 0));
    let y = f.get_node_id(Node::Variable(1, 1, 0));
    let xy = f.mul_by_id(x, y);
    let x_1 = f.add_by_id(x, 1);
    let y_1 = f.add_by_id(y, 1);
    let y_x = f.add_by_id(y, x);

    println!("{}", f.evaluate(x, &vec![0].into_iter().collect()));
    print_v(vec![x_1, x, y, y_1, y_x]);
    print_v(normal_form(&mut f, vec![x_1, x, y]));
    print_v(normal_form(&mut f, vec![xy, y]));
    println!("{:?}", f);
}
