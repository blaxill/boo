extern crate boo;

#[allow(unused_imports)]
use boo::{Forest, Node, Word, Variable};
use std::io::stdout;
use std::cell::RefCell;

type DQWord<'a> = (
    Word<'a>, // a = 0
    Word<'a>, // b = 1
    Word<'a>, // c = 2
    Word<'a>, // d = 3
    Word<'a>, // e = 4
    Word<'a>, // f = 5
    Word<'a>, // g = 6
    Word<'a>  // h = 7
    );

fn compress<'a>(forest: &'a RefCell<Forest>,
                    dq: DQWord<'a>) -> DQWord<'a> {
    let a = &dq.0;
    let b = &dq.1;
    let c = &dq.2;
    let d = &dq.3;
    let e = &dq.4;
    let f = &dq.5;
    let g = &dq.6;
    let h = &dq.7;

    let S1 = &[6, 11, 25].iter()
        .map(|&shift| e >> shift)
        .fold(Word::new(forest), |acc, item| (acc ^ item));

    let not_e = &(!e);
    let ch1 = &(e & f);
    let ch2 = &(not_e & g);
    let ch = &(ch1 ^ ch2);
    let temp1 = &(h & S1);
    let temp1 = &(temp1 + ch);

    let S0 = &[2, 13, 22].iter()
        .map(|&shift| a >> shift)
        .fold(Word::new(forest), |acc, item| (acc ^ item));
    let aNb = &(a & b);
    let aNc = &(a & c);
    let bNc = &(b & c);
    let maj = &(aNb ^ aNc);
    let maj = &(maj ^ bNc);
    let temp2 = &(S0 + maj);

    let dtemp1 = d + temp1;
    ((temp1 + temp2) , a.clone(), b.clone(), c.clone(),
    dtemp1, e.clone(), f.clone(), g.clone())
}

fn main() {
    let f = RefCell::new(Forest::with_sparsity(2));

    let a = Word::from_fn(&f, |i| {
        if i < 3 {
            f.borrow_mut().to_node_idx(Node(i as Variable, 1, 0))
        } else {
            0
        }
    });

    let _0 = Word::constant(&f, 0);
    let dq: DQWord = (a.clone(), a.clone(), a.clone(), _0.clone(), _0.clone(), _0.clone(), _0.clone(), _0);
    let res = (0..16).fold(dq, |acc, _| compress(&f, acc));

    println!("digraph {{");
    f.borrow_mut().write_graph(&mut stdout(), res.0.get_bit(0)).unwrap();
    println!("}}");
}
