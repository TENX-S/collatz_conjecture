use num_bigint::{BigUint, ToBigUint};
use num_integer::Integer;
use num_iter::range;
use num_traits::One;
use std::collections::HashSet;
use std::env;
use std::ops::Shr;
use std::time::Instant;

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
    let mut verified = [1, 2, 4]
        .iter()
        .map(|t| t.to_biguint().unwrap())
        .collect::<HashSet<_>>();

    let now = Instant::now();
    let mut current: BigUint = One::one();
    while current <= max {
        let mut target = current.clone();
        if verified.contains(&target) {
            target += 1_u32;
        } else {
            while verified.insert(target.clone()) {
                if target.is_even() {
                    target = target.shr(1);
                } else {
                    target = target * 3_u32 + 1_u32;
                }
            }
        }
        current += 1_u32;
    }
    let elapsed = now.elapsed().as_secs_f64();
    println!("{}s", elapsed);

    let test_set = range(One::one(), max + 1_u32).collect::<HashSet<_>>();
    test_set.is_subset(&verified)
}
