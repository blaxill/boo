#![feature(test)]

extern crate boo;
extern crate test;

use boo::{Variable, Node};
use boo::Forest;
use boo::Word;

use std::cell::RefCell;
use test::Bencher;

fn bench_k_sparse_64_add(b: &mut Bencher, k: usize) {
    let f = RefCell::new(Forest::with_sparsity(k));

    let mut lfsr: Variable = 57;

    let x = Word::from_fn(&f, |_| {
        lfsr = lfsr.wrapping_mul(73).wrapping_add(67);
        f.borrow_mut().to_node_idx(Node(lfsr % 64, 1, 0))
    });

    b.iter(|| {
        let y = Word::from_fn(&f, |_| {
            lfsr = lfsr.wrapping_mul(73).wrapping_add(67);
            f.borrow_mut().to_node_idx(Node(lfsr % 64, 1, 0))
        });
        &x + &y
    });
}

#[bench]
fn bench_2_sparse_64_add(b: &mut Bencher) {
    bench_k_sparse_64_add(b, 2);
}
#[bench]
fn bench_3_sparse_64_add(b: &mut Bencher) {
    bench_k_sparse_64_add(b, 3);
}
#[bench]
fn bench_4_sparse_64_add(b: &mut Bencher) {
    bench_k_sparse_64_add(b, 4);
}
#[bench]
fn bench_5_sparse_64_add(b: &mut Bencher) {
    bench_k_sparse_64_add(b, 5);
}

fn bench_k_sparse_64_xor(b: &mut Bencher, k: usize) {
    let f = RefCell::new(Forest::with_sparsity(k));

    let mut lfsr: Variable = 57;

    let x = Word::from_fn(&f, |_| {
        lfsr = lfsr.wrapping_mul(73).wrapping_add(67);
        f.borrow_mut().to_node_idx(Node(lfsr % 64, 1, 0))
    });

    b.iter(|| {
        let y = Word::from_fn(&f, |_| {
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
fn bench_16_sparse_64_xor(b: &mut Bencher) {
    bench_k_sparse_64_xor(b, 16);
}
#[bench]
fn bench_32_sparse_64_xor(b: &mut Bencher) {
    bench_k_sparse_64_xor(b, 32);
}
