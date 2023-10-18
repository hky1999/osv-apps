use std::thread;

extern crate rust_microbench;

use rust_microbench::config;
use rust_microbench::current_cycle;
use rust_microbench::pinned_on_core;

pub fn test_thread_switch(rounds: usize, warmup_rounds: usize) {
    pinned_on_core(0);
    let _handle = thread::spawn(|| {
        pinned_on_core(0);
        loop {
            thread::yield_now();
        }
    });

    let mut results = vec![];
    for i in 0..rounds + warmup_rounds {
        let start = current_cycle();
        thread::yield_now();
        let cnt = current_cycle() - start;
        if i >= warmup_rounds {
            results.push(cnt);
        }
        if config::Config::get().verbose() {
            if i % warmup_rounds == 0 || i < 10 {
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

    println!("[[TEST]] test_thread_switch {}/{rounds}", sum);
    println!("[TEST] thread finished***");
}

fn main() {
    let config = config::Config::get();
    let (rounds, warmup_rounds) = config.rounds();
    println!("[[TEST]] test_thread_switch for {rounds} rounds, warmup {warmup_rounds} rounds");
    test_thread_switch(rounds, warmup_rounds);
}
