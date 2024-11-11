//! # Custom Mutex Implementation
//!
//! This implementation is based on the Mutex tutorial from Empowered Coder cohort 8
//! Source: https://github.com/vaibhawvipul/empowered-coder-cohort/tree/main/cohort-8/mutex-tutorial
//!
//! A from-scratch implementation of a Mutex (mutual exclusion) primitive for learning purposes.

mod mutex;

use mutex::Mutex;
use std::time::Instant;

fn main() {
    println!("Hello, world! This is a mutex tutorial!");

    let regular_mutex: &'static Mutex<i32> = Box::leak(Box::new(Mutex::new(0)));
    let mutex_with_hint: &'static Mutex<i32> = Box::leak(Box::new(Mutex::new(0)));

    // Benchmark spin_lock
    let n = 10000;
    let m = 100000;
    let start = Instant::now();
    let thread_handles_spin_lock = (0..n)
        .map(|_| {
            std::thread::spawn(move || {
                regular_mutex.spin_lock(|data| {
                    for _ in 0..m {
                        *data += 1;
                    }
                })
            })
        })
        .collect::<Vec<_>>();

    for handle in thread_handles_spin_lock {
        handle.join().unwrap();
    }
    let duration_spin_lock = start.elapsed();
    let data_spin_lock = regular_mutex.spin_lock(|data| *data);
    println!("Mutex data (spin_lock): {}", data_spin_lock);
    assert!(data_spin_lock == n * m);
    println!("Time taken by spin_lock: {:?}", duration_spin_lock);

    // Benchmark spin_lock_with_hint
    let start = Instant::now();
    let thread_handles_spin_lock_with_hint = (0..n)
        .map(|_| {
            std::thread::spawn(move || {
                mutex_with_hint.spin_lock_with_hint(|data| {
                    for _ in 0..m {
                        *data += 1;
                    }
                })
            })
        })
        .collect::<Vec<_>>();

    for handle in thread_handles_spin_lock_with_hint {
        handle.join().unwrap();
    }
    let duration_spin_lock_with_hint = start.elapsed();
    let data_spin_lock_with_hint = mutex_with_hint.spin_lock_with_hint(|data| *data);
    println!(
        "Mutex data (spin_lock_with_hint): {}",
        data_spin_lock_with_hint
    );
    assert!(data_spin_lock_with_hint == n * m);
    println!(
        "Time taken by spin_lock_with_hint: {:?}",
        duration_spin_lock_with_hint
    );

    // Additional metrics
    let speedup = duration_spin_lock.as_secs_f64() / duration_spin_lock_with_hint.as_secs_f64();
    println!(
        "Speedup of spin_lock_with_hint over spin_lock: {:.2}x",
        speedup
    );

    let efficiency_spin_lock = (n * m) as f64 / duration_spin_lock.as_secs_f64();
    let efficiency_spin_lock_with_hint =
        (n * m) as f64 / duration_spin_lock_with_hint.as_secs_f64();
    println!(
        "Efficiency (spin_lock): {:.2} operations/sec",
        efficiency_spin_lock
    );
    println!(
        "Efficiency (spin_lock_with_hint): {:.2} operations/sec",
        efficiency_spin_lock_with_hint
    );

    // compare efficiency of spin_lock_with_hint over spin_lock
    let efficiency_ratio = efficiency_spin_lock_with_hint / efficiency_spin_lock;
    println!(
        "Efficiency ratio (spin_lock_with_hint / spin_lock): {:.2}x",
        efficiency_ratio
    );
}
