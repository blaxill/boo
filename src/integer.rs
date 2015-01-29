use std::collections::{HashSet};
use forest::{Forest, Node, NodeId, Term};

#[derive(Clone)]
pub struct Integer {
    bits: Vec<NodeId>,
}

impl Integer {
    pub fn new_input(f: &mut Forest, offset: Term) -> Integer {
        Integer{bits: (0u16..32)
            .map(|i| if i <16{ f.term(i+offset)} else { 0 } )
            .collect()}
    }

    pub fn new_constant(value: u32) -> Integer {
        Integer{bits: (0u16..32)
            .map(|i| ((value>>i)&1) as NodeId )
            .collect()}
    }

    pub fn as_vec(&self) -> Vec<NodeId> {
        self.bits.clone()
    }

    pub fn xor(&self, f: &mut Forest, other: &Integer) -> Integer {
        Integer{bits: (0us..32).map(|i| {
                f.add_by_id(self.bits[i], other.bits[i])
            }).collect()
        }
    }

    pub fn shl(&self, rhs: usize) -> Integer {
        Integer{bits: (0us..32).map(|i| {
                if i >= rhs { self.bits[i - rhs] }
                else { 0 }
            }).collect()
        }
    }

    pub fn add(&self, f: &mut Forest, o: &Integer) -> Integer {
        let mut bits = Vec::new();
        let mut carry = 0;

        for i in (0us..32) {
            let (lhs, rhs) = (self.bits[i], o.bits[i]);
            let xor = f.add_by_id(lhs, rhs);
            bits.push(f.add_by_id(xor,carry));
            let and = f.mul_by_id(lhs, rhs);
            let carry_and_xor = f.mul_by_id(carry, xor);
            carry = f.add_by_id(and, carry_and_xor);
        }

        Integer{ bits: bits }
    }

    pub fn mul(&self, f: &mut Forest, o: &Integer) -> Integer {
        let mut result = Integer::new_constant(0);

        for i in (0us..32){
            let mut poly = self.shl(i);

            for j in (i..32){
                poly.bits[j] = f.mul_by_id(poly.bits[j], o.bits[i]);
            }

            result = result.add(f, &poly);
        }

        result
    }

    pub fn evaluate(&self, f: &Forest, terms: &HashSet<Term>) -> u32 {
        self.bits
            .iter()
            .rev()
            .fold(0, |acc, &x|
                  (acc<<1) + if f.evaluate(x, terms) { 1 } else { 0 }
                )
    }
}

pub fn crc32_round(f: &mut Forest, input: &Integer, state: &Integer) -> Integer {
    let multiplier = Integer::new_constant(31);
    let new_state = state.mul(f, &multiplier);
    new_state.xor(f, input)
}

pub fn crc32_round_verify(input: u32, state: u32) -> u32 {
    31*state ^ input
}
