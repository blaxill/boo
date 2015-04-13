use super::forest::{Forest, NodeIdx, Variable};
use super::Cache;
use super::add::add;
use super::multiply::multiply;

use std::collections::HashSet;

#[derive(Copy, Clone)]
pub struct Word {
    bits: [NodeIdx; 32],
}

impl Word {
    fn new() -> Word {
        Word { bits: [0; 32] }
    }

    fn constant(value: u32) -> Word {
        let mut word = Word::new();

        for (idx, val) in (0..32)
            .map(|i| { ((value >> i) & 1) as usize })
            .enumerate() {
            word.bits[idx] = val;
        }

        word
    }

    fn from_fn<F>(mut func: F) -> Word
        where F: FnMut(usize) -> NodeIdx {
        let mut word = Word::new();

        for idx in 0..32 {
            word.bits[idx] = func(idx);
        }

        word
    }

    fn evaluate(&self, f: &Forest, variable_map: &HashSet<Variable>) -> u32 {
        self.bits
            .iter()
            .enumerate()
            .fold(0,
            |value, (i, &node)| {
                if f.evaluate(node, variable_map) {
                    value + (1 << i)
                } else {
                    value
                }
            })
    }

    fn add(&self,
           c: &mut Cache,
           f: &mut Forest, other: &Word) -> Word {
        let mut word = Word::new();
        let mut carry: NodeIdx = 0;

        for i in 0..32 {
            let (lhs, rhs) = (self.bits[i], other.bits[i]);
            let lhs_add_rhs = add(c, f, lhs, rhs);
            let lhs_add_rhs_add_carry = add(c, f, lhs_add_rhs, carry);
            word.bits[i] = lhs_add_rhs_add_carry;

            if i < 31 {
                let mul_carry = multiply(c, f, lhs_add_rhs, carry);
                let lhs_mul_rhs = multiply(c, f, lhs, rhs);
                carry = add(c, f, lhs_mul_rhs, mul_carry);
            }
        }

        word
    }

    fn xor(&self,
           c: &mut Cache,
           f: &mut Forest, other: &Word) -> Word {
        let mut word = Word::new();

        for i in 0..32 {
            let (lhs, rhs) = (self.bits[i], other.bits[i]);
            word.bits[i] = add(c, f, lhs, rhs);
        }

        word
    }

    fn and(&self,
           c: &mut Cache,
           f: &mut Forest, other: &Word) -> Word {
        let mut word = Word::new();

        for i in 0..32 {
            let (lhs, rhs) = (self.bits[i], other.bits[i]);
            word.bits[i] = multiply(c, f, lhs, rhs);
        }

        word
    }

    fn right_rotate(&self,
           c: &mut Cache,
           f: &mut Forest, distance: usize) -> Word {
        let mut word = Word::new();
        for i in 0..32 {
            word.bits[i] = self.bits[(i+distance)%32];
        }
        word
    }

    fn not(&self,
           c: &mut Cache,
           f: &mut Forest) -> Word {
        let mut word = Word::new();
        for i in 0..32 {
            word.bits[i] = add(c, f, self.bits[i], 1);
        }
        word
    }
}

pub fn compress(cache: &mut Cache,
            forest: &mut Forest,
            mut a: Word, mut b: Word, mut c: Word, mut d: Word,
            mut e: Word, mut f: Word, mut g: Word, mut h: Word) ->
(Word, Word, Word, Word, Word, Word, Word, Word) {

    println!("compressing...");
    for i in 0..64 {
        //print!("compress {} of 63... ", i);
        let S1 = [6, 11, 25].iter()
            .map(|&shift| e.right_rotate(cache, forest, shift)).collect::<Vec<_>>().iter()
            .fold(Word::new(), |acc, item| acc.xor(cache, forest, &item));
        let ch1 = e.and(cache, forest, &f);
        let ch2 = e.not(cache, forest).and(cache, forest, &g);
        let ch = ch1.xor(cache, forest, &ch2);
        let temp1 = h.add(cache, forest, &S1)
            .add(cache, forest, &ch);
            //.add(c, forest, k[i]);
            //.add(c, forest, w[i]);

        let S0 = [2, 13, 22].iter()
            .map(|&shift| a.right_rotate(cache, forest, shift)).collect::<Vec<_>>().iter()
            .fold(Word::new(), |acc, item| acc.xor(cache, forest, &item));
        let aNb = a.and(cache, forest, &b);
        let aNc = a.and(cache, forest, &c);
        let bNc = b.and(cache, forest, &c);
        let maj = aNb.xor(cache, forest, &aNc).xor(cache, forest, &bNc);
        let temp2 = S0.add(cache, forest, &maj);

        h = g;
        g = f;
        f = e;
        e = d.add(cache, forest, &temp1);
        d = c;
        c = b;
        b = a;
        a = temp1.add(cache, forest, &temp2);
    }
    (a, b, c, d, e, f, g, h)
}

#[cfg(test)]
mod test {
    extern crate test;

    use super::*;
    use super::super::forest::{Forest, Node, Variable};
    use super::super::Cache;
    use std::collections::HashSet;

    use self::test::Bencher;

    #[test]
    fn word_basic() {
        let f = &mut Forest::with_sparsity(4);
        let c = &mut Cache::new();

        let x = Word::constant(4);
        let y = Word::constant(5);
        let z = Word::constant(0x1337);
        let w = Word::constant(0xC0DEC0DE);

        assert_eq!(x.evaluate(f, &HashSet::new()), 4);
        assert_eq!(y.evaluate(f, &HashSet::new()), 5);
        assert_eq!(z.evaluate(f, &HashSet::new()), 0x1337);
        assert_eq!(w.evaluate(f, &HashSet::new()), 0xC0DEC0DE);
    }

    #[test]
    fn word_poly() {
        let f = &mut Forest::with_sparsity(4);
        let c = &mut Cache::new();

        let x = Word::from_fn(|i|{
            if i < 4 {
                f.to_node_idx(Node(i as Variable, 1, 0))
            } else { 0 }
        });
        let y = Word::from_fn(|i|{
            if i < 4 {
                f.to_node_idx(Node((i + 4) as Variable, 1, 0))
            } else { 0 }
        });
        let z = Word::constant(0x1337);
        let w = Word::constant(0xC0DEC0DE);

        let all_set: HashSet<Variable> = (0..8).collect();
        let none_set: HashSet<Variable> = HashSet::new();

        assert_eq!(x.evaluate(f, &all_set), 15);
        assert_eq!(y.evaluate(f, &all_set), 15);
        assert_eq!(x.evaluate(f, &none_set), 0);
        assert_eq!(y.evaluate(f, &none_set), 0);
        assert_eq!(z.evaluate(f, &HashSet::new()), 0x1337);
        assert_eq!(w.evaluate(f, &HashSet::new()), 0xC0DEC0DE);
    }

    #[bench]
    fn bench_compress(bench: &mut Bencher) {
        let forest = &mut Forest::with_sparsity(3);
        let cache = &mut Cache::new();

        let mut lfsr: Variable = 1337;

        macro_rules! gen(() =>
                         (Word::from_fn(|i|{
                             lfsr = lfsr * 3138121 + 130371;
                             forest.to_node_idx(Node(lfsr % 8, 1, 0))
                         }))
                        );
        let a = gen!();
        let b = gen!();
        let c = gen!();
        let d = gen!();
        let e = gen!();
        let f = gen!();
        let g = gen!();
        let h = gen!();

        bench.iter(|| {
            compress(cache, forest, a, b, c, d, e, f, g, h)
        });
    }

    fn bench_k_sparse_64_add(b: &mut Bencher, k: usize) {
        let f = &mut Forest::with_sparsity(k);
        let c = &mut Cache::new();

        let mut lfsr: Variable = 1337;

        let mut x = Word::from_fn(|i|{
            lfsr = lfsr * 3138121 + 130371;
            f.to_node_idx(Node(lfsr % 16, 1, 0))
        });

        b.iter(|| {
            let y = Word::from_fn(|i|{
                lfsr = lfsr * 3138121 + 130371;
                f.to_node_idx(Node(lfsr % 16, 1, 0))
            });
            x.add(c, f, &y)
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
        let f = &mut Forest::with_sparsity(k);
        let c = &mut Cache::new();

        let mut lfsr: Variable = 1337;

        let mut x = Word::from_fn(|i|{
            lfsr = lfsr * 3138121 + 130371;
            f.to_node_idx(Node(lfsr % 64, 1, 0))
        });

        b.iter(|| {
            let y = Word::from_fn(|i|{
                lfsr = lfsr * 3138121 + 130371;
                f.to_node_idx(Node(lfsr % 64, 1, 0))
            });

            x = x.xor(c, f, &y);
            x
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
}
