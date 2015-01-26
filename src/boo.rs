use forest::{Forest, Node, NodeId};
use grobner::normal_form;
use integer::{Integer, crc32_round, crc32_round_verify};

mod forest;
mod grobner;
mod integer;

fn print_v(v: &Vec<usize>) {
    print!("[");
    for e in v.iter() {
        print!("{}, ", e);
    }
    println!("]");
}

fn chain_crc32_verify(m: Vec<u32>) -> u32 {
    m.into_iter()
        .fold(0, |o, x| crc32_round_verify(x, o))
}

fn chain_crc32(f: &mut Forest, m: Vec<Integer>) -> Integer {
    m.iter()
        .fold(
            Integer::new_constant(0),
            |o, x| crc32_round(f, x, &o))
}

fn crc32(f: &mut Forest, len: u16, output: Integer) -> Vec<NodeId> {
    (0..len).flat_map(|x| {
        let input = Integer::new_input(f, x*32);
        let state = Integer::new_input(f, x*32 + len*32);
        let output_state = Integer::new_input(f, (x+1)*32 + len*32);
        let output = crc32_round(f, &input, &state);

        println!("crc32 step {} of {}", x+1, len);

        if x == len-1 {
            let mut s = output.xor(f, &output_state).as_vec();
            let mut o = output_state.xor(f, &output).as_vec();
            s.append(&mut o);
            s
        } else {
            output.xor(f, &output_state).as_vec()
        }.into_iter()
    }).collect()
}

fn main(){
    let mut f = Forest::new();

    let equations = crc32(&mut f, 2, Integer::new_constant(123));
    let reduced = normal_form(&mut f, equations.clone());
    println!("{:?} - {}", equations, equations.len());

    println!("{:?} - {}", reduced, reduced.len());
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

    /*let input = (0..2).map(|x| Integer::new_input(&mut f, x*32)).collect();

    let real_output = chain_crc32_verify(vec![16, 1]);
    let mut value = chain_crc32(&mut f, input);

    let eqs = value.xor(&mut f, &Integer::new_constant(real_output));
    let terms = eqs.as_vec()
        .iter()
        .filter_map(|&x| 
                    match f.is_term_equation(x) {
                        Some((term, true)) => Some(term),
                        _ => None,
                    }).collect();
    println!("{:?}", real_output);
    println!("{:?} | {}", value.evaluate(&f, &terms), terms.len());
    println!("{:?}", eqs.evaluate(&f, &vec![].into_iter().collect()));
    print_v(eqs.as_vec());
    //print_v(normal_form(&mut f, eqs.as_vec()));*/
}

