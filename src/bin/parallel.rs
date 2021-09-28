use mimalloc::MiMalloc;
use num_bigint::{BigUint, ToBigUint};
use num_integer::Integer;
use num_iter::range;
use num_traits::One;
use parking_lot::Mutex;
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
    if verify(max.clone()) {
        println!("The numbers in the range 1 to {} agree with the guess", max);
    } else {
        println!("WTF");
    }
}

fn verify(max: BigUint) -> bool {
    let verified = Mutex::new(
        [1, 2, 4]
            .iter()
            .map(|t| t.to_biguint().unwrap())
            .collect::<HashSet<_>>(),
    );

    let now = Instant::now();
    let all = range(One::one(), max.clone() + 1_u32)
        .rev()
        .collect::<Vec<_>>();

    all.into_par_iter().for_each(|t| {
        let mut verified_lock = verified.lock();
        let mut current = t.clone();
        while verified_lock.insert(current.clone()) {
            if current.is_even() {
                current = current.shr(1);
            } else {
                current = current * 3_u32 + 1_u32;
            }
        }
    });
    let elapsed = now.elapsed().as_secs_f64();
    println!("{}s", elapsed);

    let test_set = range(One::one(), max + 1_u32).collect::<HashSet<_>>();
    let result = test_set.is_subset(&*verified.lock());
    result
}
