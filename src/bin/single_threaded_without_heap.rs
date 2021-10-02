use mimalloc::MiMalloc;
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

fn verify(max: u32) -> bool {
    let mut verified = HashSet::new();
    for i in [1, 2, 4] {
        verified.insert(i);
    }

    let now = Instant::now();
    let mut current = 1;
    while current <= max {
        let mut target = current;
        if verified.contains(&target) {
            target += 1;
        } else {
            while verified.insert(target) {
                if target % 2 == 0 {
                    target = target.shr(1);
                } else {
                    target = target * 3 + 1;
                }
            }
        }
        current += 1;
    }
    let elapsed = now.elapsed().as_secs_f64();
    println!("{}s", elapsed);

    let test_set = (1..max+1).into_iter().collect::<HashSet<_>>();
    test_set.is_subset(&verified)
}
