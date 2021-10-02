use mimalloc::MiMalloc;
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
        .unwrap();
    verify(max);
}

fn verify(max: u32) {
    let verified = [1, 2, 4]
        .iter()
        .collect::<HashSet<_>>();

    let now = Instant::now();
    let all = (1..max+1)
        .rev()
        .collect::<Vec<_>>();

    all.into_par_iter().for_each(|t| {
        let mut current = t.clone();
        while !verified.contains(&current) {
            if current % 2 == 0 {
                current = current.shr(1);
            } else {
                current = current * 3 + 1;
            }
        }
    });

    let elapsed = now.elapsed().as_secs_f64();
    println!("{}s", elapsed);
}
