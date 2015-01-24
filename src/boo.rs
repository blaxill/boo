use forest::{Forest, Node};
use grobner::normal_form;
use integer::{Integer, crc32_round, crc32_round_verify};

mod forest;
mod grobner;
mod integer;

fn print_v(v: Vec<usize>) {
    for e in v.iter() {
        print!("{}, ", e);
    }
    println!("");
}

fn main(){
    let mut f = Forest::new();
    /*let x = f.get_node_id(Node::Variable(0, 1, 0));
    let y = f.get_node_id(Node::Variable(1, 1, 0));
    let xy = f.mul_by_id(x, y);
    let x_1 = f.add_by_id(x, 1);
    let y_1 = f.add_by_id(y, 1);
    let y_x = f.add_by_id(y, x);

    println!("{}", f.evaluate(x, &vec![0].into_iter().collect()));
    print_v(vec![x_1, x, y, y_1, y_x]);
    print_v(normal_form(&mut f, vec![x_1, x, y]));
    print_v(normal_form(&mut f, vec![xy, y_1]));
    println!("{:?}", f);*/

    let real_output = crc32_round_verify(20, 345);
    let input = Integer::new_input(&mut f, 0);
    let mut value = crc32_round(&mut f,
                            &input,
                            &Integer::new_constant(345));
    let eqs = value.xor(&mut f, &Integer::new_constant(real_output));
    let terms = eqs.as_vec()
        .iter()
        .filter_map(|&x| 
                    match f.is_term_equation(x) {
                        Some((term, true)) => Some(term),
                        _ => None,
                    }).collect();
    println!("{:?}", real_output);
    println!("{:?}", value.evaluate(&f, &terms));
    println!("{:?}", eqs.evaluate(&f, &vec![].into_iter().collect()));
    print_v(eqs.as_vec());
    print_v(normal_form(&mut f, eqs.as_vec()));
}

