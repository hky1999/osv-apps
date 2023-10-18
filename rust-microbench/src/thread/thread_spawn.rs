use std::thread;

extern crate core_affinity;
extern crate rust_microbench;

use rust_microbench::config;
use rust_microbench::current_cycle;
use rust_microbench::pinned_on_core;

pub fn test_thread_spawn(rounds: usize, warmup_rounds: usize) {
    pinned_on_core(0);

    let mut results = vec![];
    for i in 0..rounds + warmup_rounds {
        let start = current_cycle();
        let _handle = thread::spawn(move || {
            // let res = core_affinity::set_for_current(pin_core);
        });
        let cnt = current_cycle() - start;
        thread::yield_now();

        drop(_handle);

        if i >= warmup_rounds {
            results.push(cnt);
        }
        if config::Config::get().verbose() {
            if i % 1000 == 0 || i < 10 {
                println!(
                    "main thread, round [{}] {} cycles start {} end {}",
                    i,
                    cnt,
                    start,
                    start + cnt
                );
            }
        }
    }

    let mut sum = 0;
    for result in results {
        // println!("[{}] result {} cycle", i, result);
        sum += result;
    }

    println!("[[TEST]] test_thread_spawn {}/{rounds}", sum);
    println!("[TEST] thread finished***");
}

fn main() {
    let config = config::Config::get();
    let (rounds, warmup_rounds) = config.rounds();
    println!("[[TEST]] test_thread_spawn for {rounds} rounds, warmup {warmup_rounds} rounds");
    test_thread_spawn(rounds, warmup_rounds);
}
