use super::node::{NodeIdx, Variable};
use super::forest::Forest;
use super::add::add;
use super::multiply::multiply;

use std::collections::HashSet;
use std::cell::RefCell;
use std::hash::{Hash, Hasher};

use std::ops::{Add, BitAnd, BitXor, Not, Shr};

#[derive(Clone, Debug)]
pub struct Word<'a> {
    forest: &'a RefCell<Forest>,
    bits: [NodeIdx; 32],
}

impl<'a> PartialEq for Word<'a> {
    fn eq(&self, other: &Word<'a>) -> bool {
        for i in 0..32 {
            if self.bits[i] != other.bits[i] {
                return false;
            }
        }
        return true;
    }
}

impl<'a> Eq for Word<'a> {}

impl<'a> Hash for Word<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for i in 0..32 {
            state.write_usize(self.bits[i]);
        }
    }
}

impl<'a> Word<'a> {
    pub fn new(forest: &RefCell<Forest>) -> Word {
        Word {
            forest: forest,
            bits: [0; 32],
        }
    }

    pub fn constant(forest: &RefCell<Forest>, value: u32) -> Word {
        let mut word = Word::new(forest);

        for (idx, val) in (0..32)
                              .map(|i| ((value >> i) & 1) as usize)
                              .enumerate() {
            word.bits[idx] = val;
        }

        word
    }

    pub fn from_fn<F>(forest: &RefCell<Forest>, mut func: F) -> Word
        where F: FnMut(usize) -> NodeIdx
    {
        let mut word = Word::new(forest);

        for idx in 0..32 {
            word.bits[idx] = func(idx);
        }

        word
    }

    pub fn get_bit(&self, bit: usize) -> NodeIdx {
        self.bits[bit]
    }

    pub fn evaluate<'b, 'c>(&self, variable_map: &'b HashSet<Variable>) -> u32 {
        self.bits
            .iter()
            .enumerate()
            .fold(0, |value, (i, &node)| {
                println!("{}", self.forest.borrow().evaluate(node, variable_map));
                if self.forest.borrow().evaluate(node, variable_map) {
                    value + (1 << i)
                } else {
                    value
                }
            })
    }
}

impl<'a, 'b, 'c> Add<&'c Word<'a>> for &'b Word<'a> {
    type Output = Word<'a>;

    fn add(self, other: &Word<'a>) -> Word<'a> {
        let mut word = Word::new(self.forest);
        let mut carry: NodeIdx = 0;
        let mut f = self.forest.borrow_mut();

        for i in 0..32 {
            let (lhs, rhs) = (self.bits[i], other.bits[i]);
            let lhs_add_rhs = add(&mut f, lhs, rhs);
            let lhs_add_rhs_add_carry = add(&mut f, lhs_add_rhs, carry);
            word.bits[i] = lhs_add_rhs_add_carry;

            if i < 31 {
                let mul_carry = multiply(&mut f, lhs_add_rhs, carry);
                let lhs_mul_rhs = multiply(&mut f, lhs, rhs);
                carry = add(&mut f, lhs_mul_rhs, mul_carry);
            }
        }

        word
    }
}

impl<'a, 'b> BitXor<Word<'a>> for Word<'a> {
    type Output = Word<'a>;

    fn bitxor(self, other: Word<'a>) -> Word<'a> {
        let mut word = Word::new(self.forest);
        let mut f = self.forest.borrow_mut();

        for i in 0..32 {
            let (lhs, rhs) = (self.bits[i], other.bits[i]);
            word.bits[i] = add(&mut f, lhs, rhs);
        }

        word
    }
}

impl<'a, 'b, 'c> BitXor<&'c Word<'a>> for &'b Word<'a> {
    type Output = Word<'a>;

    fn bitxor(self, other: &Word<'a>) -> Word<'a> {
        let mut word = Word::new(self.forest);
        let mut f = self.forest.borrow_mut();

        for i in 0..32 {
            let (lhs, rhs) = (self.bits[i], other.bits[i]);
            word.bits[i] = add(&mut f, lhs, rhs);
        }

        word
    }
}

impl<'a, 'b, 'c> BitAnd<&'c Word<'a>> for &'b Word<'a> {
    type Output = Word<'a>;

    fn bitand(self, other: &Word<'a>) -> Word<'a> {
        let mut word = Word::new(self.forest);
        let mut f = self.forest.borrow_mut();

        for i in 0..32 {
            let (lhs, rhs) = (self.bits[i], other.bits[i]);
            word.bits[i] = multiply(&mut f, lhs, rhs);
        }

        word
    }
}

impl<'a, 'b> Shr<usize> for &'b Word<'a> {
    type Output = Word<'a>;

    fn shr(self, distance: usize) -> Word<'a> {
        let mut word = Word::new(self.forest);

        for i in 0..32 {
            word.bits[i] = self.bits[(i + distance) % 32];
        }

        word
    }
}

impl<'a, 'b> Not for &'b Word<'a> {
    type Output = Word<'a>;

    fn not(self) -> Word<'a> {
        let mut word = Word::new(self.forest);
        let mut f = self.forest.borrow_mut();

        for i in 0..32 {
            word.bits[i] = add(&mut f, self.bits[i], 1);
        }

        word
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::node::{Node, Variable};
    use super::super::forest::Forest;

    use std::collections::HashSet;
    use std::cell::RefCell;

    #[test]
    fn word_basic() {
        let f = RefCell::new(Forest::with_sparsity(4));

        let x = Word::constant(&f, 4);
        let y = Word::constant(&f, 5);
        let z = Word::constant(&f, 0x138);
        let w = Word::constant(&f, 0xC0DEC0DE);

        assert_eq!(x.evaluate(&HashSet::new()), 4);
        assert_eq!(y.evaluate(&HashSet::new()), 5);
        assert_eq!(z.evaluate(&HashSet::new()), 0x138);
        assert_eq!(w.evaluate(&HashSet::new()), 0xC0DEC0DE);

        let x_y = &x + &y;
        assert_eq!(x_y.evaluate(&HashSet::new()), 9);
    }


    #[test]
    fn word_poly() {
        let f = RefCell::new(Forest::with_sparsity(4));

        let x = Word::from_fn(&f, |i| {
            if i < 4 {
                f.borrow_mut().to_node_idx(Node(i as Variable, 1, 0))
            } else {
                0
            }
        });
        let y = Word::from_fn(&f, |i| {
            if i < 4 {
                f.borrow_mut().to_node_idx(Node((i + 4) as Variable, 1, 0))
            } else {
                0
            }
        });
        let z = Word::constant(&f, 0x147);
        let w = Word::constant(&f, 0xC0DEC0DE);

        let all_set: HashSet<Variable> = (0..8).collect();
        let none_set: HashSet<Variable> = HashSet::new();

        assert_eq!(x.evaluate(&all_set), 15);
        assert_eq!(y.evaluate(&all_set), 15);
        assert_eq!(x.evaluate(&none_set), 0);
        assert_eq!(y.evaluate(&none_set), 0);
        assert_eq!(z.evaluate(&HashSet::new()), 0x147);
        assert_eq!(w.evaluate(&HashSet::new()), 0xC0DEC0DE);
    }
}
