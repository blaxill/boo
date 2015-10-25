use super::forest::{Forest, NodeIdx, Variable, Node};
use super::add::add;
use super::multiply::multiply;
//use super::monomial_count::monomial_count;

use std::collections::HashSet;
use std::cell::RefCell;
use std::hash::{Hash, Hasher};

use std::ops::{Add, BitAnd, BitXor, Not, Shr, FnOnce};

use test::Bencher;

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

impl<'a> Eq for Word<'a> { }

impl<'a> Hash for Word<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for i in 0..32 {
            state.write_usize(self.bits[i]);
        }
    }
}

impl<'a> Word<'a> {
    pub fn new(forest: &RefCell<Forest>) -> Word {
        Word { forest: forest, bits: [0; 32] }
    }

    pub fn constant(forest: &RefCell<Forest>, value: u32) -> Word {
        let mut word = Word::new(forest);

        for (idx, val) in (0..32)
            .map(|i| { ((value >> i) & 1) as usize })
            .enumerate() {
            word.bits[idx] = val;
        }

        word
    }

    pub fn from_fn<F>(forest: &RefCell<Forest>, mut func: F) -> Word
        where F: FnMut(usize) -> NodeIdx {
        let mut word = Word::new(forest);

        for idx in 0..32 {
            word.bits[idx] = func(idx);
        }

        word
    }

    #[allow(unreachable_code)]
    pub fn approximate_jacobian<'b>(&self) -> Box<Fn(&Word<'b>,Word<'b>)->Word<'b> + 'a> {
        panic!("Not fully implemented.");
        let forest = self.forest;
        let words: Vec<Word<'a>> = self.bits.iter().map(|_bit| {
            let word = Word::from_fn(forest, |_i|{
                let dbit_di = 0;//bit.partial_derivative(i);
                if forest.borrow().evaluate(dbit_di, &HashSet::new()) { 1 }
                else { 0 }
            });
            word
        }).collect();
        return Box::new(move |input, _| {
            words.iter().map(|_|0).fold(0,|_,_|0);
            Word::new(input.forest)
        });
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
            word.bits[i] = self.bits[(i+distance)%32];
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

impl<'a, 'b, 'c> Fn<(&'b HashSet<Variable>,)> for Word<'a> {
    extern "rust-call" fn call(&self, args: (&'b HashSet<Variable>,)) -> u32 {
        self.call_once(args)
    }
}

impl<'a, 'b, 'c> FnMut<(&'b HashSet<Variable>,)> for Word<'a> {
    extern "rust-call" fn call_mut(&mut self, args: (&'b HashSet<Variable>,)) -> u32 {
        self.call_once(args)
    }
}

impl<'a, 'b, 'c> FnOnce<(&'b HashSet<Variable>,)> for Word<'a> {
    type Output = u32;
    extern "rust-call" fn call_once(self, (variable_map,): (&'b HashSet<Variable>,)) -> u32 {
        self.bits
            .iter()
            .enumerate()
            .fold(0,
            |value, (i, &node)| {
                if self.forest.borrow().evaluate(node, variable_map) {
                    value + (1 << i)
                } else {
                    value
                }
            })
    }
}


    #[test]
    fn word_basic() {
        let f = RefCell::new(Forest::with_sparsity(4));

        let x = Word::constant(&f, 4);
        let y = Word::constant(&f, 5);
        let z = Word::constant(&f, 0x138);
        let w = Word::constant(&f, 0xC0DEC0DE);

        assert_eq!(x(&HashSet::new()), 4);
        assert_eq!(y(&HashSet::new()), 5);
        assert_eq!(z(&HashSet::new()), 0x138);
        assert_eq!(w(&HashSet::new()), 0xC0DEC0DE);

        let x_y = &x + &y;
        assert_eq!(x_y(&HashSet::new()), 9);
    }


    #[test]
    fn word_poly() {
        let f = RefCell::new(Forest::with_sparsity(4));

        let x = Word::from_fn(&f, |i|{
            if i < 4 {
                f.borrow_mut().to_node_idx(Node(i as Variable, 1, 0))
            } else { 0 }
        });
        let y = Word::from_fn(&f, |i|{
            if i < 4 {
                f.borrow_mut().to_node_idx(Node((i + 4) as Variable, 1, 0))
            } else { 0 }
        });
        let z = Word::constant(&f, 0x147);
        let w = Word::constant(&f, 0xC0DEC0DE);

        let all_set: HashSet<Variable> = (0..8).collect();
        let none_set: HashSet<Variable> = HashSet::new();

        assert_eq!(x(&all_set), 15);
        assert_eq!(y(&all_set), 15);
        assert_eq!(x(&none_set), 0);
        assert_eq!(y(&none_set), 0);
        assert_eq!(z(&HashSet::new()), 0x147);
        assert_eq!(w(&HashSet::new()), 0xC0DEC0DE);
    }

    fn bench_k_sparse_64_add(b: &mut Bencher, k: usize) {
        let f = RefCell::new(Forest::with_sparsity(k));

        let mut lfsr: Variable = 57;

        let x = Word::from_fn(&f, |_|{
            lfsr = lfsr.wrapping_mul(73).wrapping_add(67);
            f.borrow_mut().to_node_idx(Node(lfsr % 16, 1, 0))
        });

        b.iter(|| {
            let y = Word::from_fn(&f, |_|{
                lfsr = lfsr.wrapping_mul(73).wrapping_add(67);
                f.borrow_mut().to_node_idx(Node(lfsr % 16, 1, 0))
            });
            &x + &y
        });
    }

    #[bench]
    fn bench_8_sparse_64_add(b: &mut Bencher) {
        bench_k_sparse_64_add(b, 2);
    }
    #[bench]
    fn bench_10_sparse_64_add(b: &mut Bencher) {
        bench_k_sparse_64_add(b, 4);
    }

    fn bench_k_sparse_64_xor(b: &mut Bencher, k: usize) {
        let f = RefCell::new(Forest::with_sparsity(k));

        let mut lfsr: Variable = 57;

        let x = Word::from_fn(&f, |_|{
            lfsr = lfsr.wrapping_mul(73).wrapping_add(67);
            f.borrow_mut().to_node_idx(Node(lfsr % 64, 1, 0))
        });

        b.iter(|| {
            let y = Word::from_fn(&f, |_|{
                lfsr = lfsr.wrapping_mul(73).wrapping_add(67);
                f.borrow_mut().to_node_idx(Node(lfsr % 64, 1, 0))
            });

            &x ^ &y
        });
    }

    #[bench]
    fn bench_8_sparse_64_xor(b: &mut Bencher) {
        bench_k_sparse_64_xor(b, 8);
    }
    #[bench]
    fn bench_10_sparse_64_xor(b: &mut Bencher) {
        bench_k_sparse_64_xor(b, 10);
    }
    #[bench]
    fn bench_12_sparse_64_xor(b: &mut Bencher) {
        bench_k_sparse_64_xor(b, 12);
    }
