#![feature(allocator_api)]

use std::alloc::{Allocator, Global, Layout};

extern crate rust_microbench;

use rust_microbench::config;
use rust_microbench::current_cycle;
use rust_microbench::pinned_on_core;

const PAGE_SIZE: usize = 4096;

pub fn test_mm_alloc_zeroed(rounds: usize, warmup_rounds: usize, bytes: usize) {
    let mut results = vec![];
    let mut results_1 = vec![];
    for i in 0..rounds + warmup_rounds {
        let layout = Layout::from_size_align(bytes, PAGE_SIZE).unwrap();
        // let layout = Layout::new::<i32>();
        let mut res_list = vec![];
        let cnt: usize;
        let cnt_1: usize;
        let mut cnt_inside = 0;
        for _j in 0..100 {
            let start = current_cycle();
            let res = Global.allocate_zeroed(layout);
            let end = current_cycle();
            res_list.push(res);
            cnt_inside = cnt_inside + (end - start);
        }
        cnt = cnt_inside / 100;
        // println!("main thread, round [{}] allocate {} cycles", i, cnt);
        cnt_inside = 0;
        for res in res_list {
            let ptr = res.unwrap();
            unsafe {
                let start = current_cycle();
                Global.deallocate(ptr.cast(), layout);
                let end = current_cycle();
                cnt_inside = cnt_inside + (end - start);
            }
        }
        cnt_1 = cnt_inside / 100;
        // println!("main thread, round [{}] deallocate {} cycles", i, cnt_1);

        if i >= warmup_rounds {
            results.push(cnt);
            results_1.push(cnt_1);
        }
        if config::Config::get().verbose() {
            if i % 1000 == 0 || i < 10 {
                println!(
                    "main thread, round [{}] allocate {} cycles deallocate {} cycles",
                    i, cnt, cnt_1
                );
            }
        }
    }

    let mut sum = 0;
    for result in results {
        // println!("[{}] result {} cycle", i, result);
        sum += result;
    }

    let mut sum_1 = 0;
    for result in results_1 {
        // println!("[{}] result {} cycle", i, result);
        sum_1 += result;
    }

    println!("[[TEST]] test_mm allocate {}/{rounds}", sum);
    println!("[[TEST]] test_mm deallocate {}/{rounds}", sum_1);
}

fn main() {
    let config = config::Config::get();
    let (rounds, warmup_rounds) = config.rounds();
    let bytes = config.bytes();
    println!("[[TEST]] test_mm_alloc_zeroed {bytes} bytes for {rounds} rounds, warmup {warmup_rounds} rounds");

    pinned_on_core(0);
    test_mm_alloc_zeroed(rounds, warmup_rounds, bytes);

    println!("[TEST] test_mm_alloc_zeroed finished***");
}
