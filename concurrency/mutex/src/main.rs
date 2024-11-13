//! # Custom Mutex Implementation
//!
//! This implementation is based on the Mutex tutorial from Empowered Coder cohort 8
//! Source: https://github.com/vaibhawvipul/empowered-coder-cohort/tree/main/cohort-8/mutex-tutorial
//!
//! A from-scratch implementation of a Mutex (mutual exclusion) primitive for learning purposes.

mod mutex;

use mutex::Mutex;
use std::{sync::Arc, time::Instant};

fn run_mutex(name: &str, n: usize, m: usize) -> i32 {
    // I don't know how to pass by parameter the lock function
    let lock_fn = match name {
        "spin_lock" => Mutex::spin_lock,
        "spin_lock_with_hint" => Mutex::spin_lock_with_hint,
        "spin_lock_with_hint_backoff" => Mutex::spin_lock_with_hint_backoff,
        "spin_lock_with_park" => Mutex::spin_lock_with_park,
        _ => panic!("Invalid mutex implementation: {}", name),
    };

    let mutex = Arc::new(Mutex::new(0));
    let threads: Vec<_> = (0..n)
        .map(|_| {
            let mutex = Arc::clone(&mutex);
            std::thread::spawn(move || {
                lock_fn(&mutex, move |data: &mut i32| {
                    for _ in 0..m {
                        *data += 1;
                    }
                    *data
                })
            })
        })
        .collect();

    // Join threads and handle errors more gracefully
    for handle in threads {
        if let Err(e) = handle.join() {
            eprintln!("Thread panicked: {:?}", e);
        }
    }

    // I don't know how to get away from this borrow checker error
    let lock_fn = match name {
        "spin_lock" => Mutex::spin_lock,
        "spin_lock_with_hint" => Mutex::spin_lock_with_hint,
        "spin_lock_with_hint_backoff" => Mutex::spin_lock_with_hint_backoff,
        "spin_lock_with_park" => Mutex::spin_lock_with_park,
        _ => panic!("Invalid mutex implementation: {}", name),
    };

    lock_fn(&mutex, |data: &mut i32| *data)
}

fn run_benchmark(name: &str, n: usize, m: usize) -> (std::time::Duration, i32) {
    let start = Instant::now();
    let data = run_mutex(name, n, m);
    let duration = start.elapsed();
    (duration, data)
}

fn main() {
    println!("Hello, world! This is a mutex tutorial!");

    const N: usize = 10000;
    const M: usize = 100000;

    let mutex_implementations = [
        "spin_lock",
        "spin_lock_with_hint",
        "spin_lock_with_hint_backoff",
        "spin_lock_with_park",
    ];

    for &name in &mutex_implementations {
        let (duration, eff) = run_benchmark(name, N, M);
        println!("{}: {:?}, {}", name, duration, eff);
    }
}
