use mimalloc::MiMalloc;
use num_bigint::{BigUint, ToBigUint};
use num_integer::Integer;
use num_iter::range;
use num_traits::One;
use rayon::prelude::*;
use std::collections::HashSet;
use std::env;
use std::ops::Shr;
use std::time::Instant;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        panic!("Should accept one maximum value");
    }
    let max = args
        .last()
        .unwrap()
        .parse::<u32>()
        .unwrap()
        .to_biguint()
        .unwrap();
    verify(max.clone());
}

fn verify(max: BigUint) {
    let verified = [1, 2, 4]
        .iter()
        .map(|t| t.to_biguint().unwrap())
        .collect::<HashSet<_>>();

    let now = Instant::now();
    let all = range(One::one(), max.clone() + 1_u32)
        .rev()
        .collect::<Vec<_>>();

    all.into_par_iter().for_each(|t| {
        let mut current = t.clone();
        while !verified.contains(&current) {
            if current.is_even() {
                current = current.shr(1);
            } else {
                current = current * 3_u32 + 1_u32;
            }
        }
    });

    let elapsed = now.elapsed().as_secs_f64();
    println!("{}s", elapsed);
}
