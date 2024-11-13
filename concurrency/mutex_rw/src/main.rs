use rand::Rng;
use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{sync::Arc, thread, time::Duration};

pub struct RwSpinLock<T> {
    // Uses a counter where:
    // 0 = unlocked
    // 1 = write locked
    // n > 1 = (n-1) read locks
    state: AtomicUsize,
    data: UnsafeCell<T>,
}

unsafe impl<T: Send> Send for RwSpinLock<T> {}
unsafe impl<T: Send> Sync for RwSpinLock<T> {}

impl<T> RwSpinLock<T> {
    pub fn new(data: T) -> Self {
        Self {
            state: AtomicUsize::new(0),
            data: UnsafeCell::new(data),
        }
    }

    pub fn read<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&T) -> R,
    {
        // Try to acquire read lock
        loop {
            let state = self.state.load(Ordering::Relaxed);
            if state != 1 {
                // Not write locked
                match self.state.compare_exchange(
                    state,
                    state + 1,
                    Ordering::Acquire,
                    Ordering::Relaxed,
                ) {
                    Ok(_) => break,
                    Err(_) => continue,
                }
            }
            std::hint::spin_loop();
        }

        // Execute closure with immutable data
        let result = f(unsafe { &*self.data.get() });

        // Release read lock
        self.state.fetch_sub(1, Ordering::Release);

        result
    }

    pub fn write<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut T) -> R,
    {
        // Try to acquire write lock
        loop {
            match self
                .state
                .compare_exchange(0, 1, Ordering::Acquire, Ordering::Relaxed)
            {
                Ok(_) => break,
                Err(_) => {
                    while self.state.load(Ordering::Relaxed) != 0 {
                        std::hint::spin_loop();
                    }
                }
            }
        }

        // Execute closure with mutable data
        let result = f(unsafe { &mut *self.data.get() });

        // Release write lock
        self.state.store(0, Ordering::Release);

        result
    }
}

fn main() {
    let lock = Arc::new(RwSpinLock::new(0i32));
    let mut handles = Vec::new();

    // Spawn 4 writer threads
    for i in 0..4 {
        let lock = Arc::clone(&lock);
        handles.push(thread::spawn(move || {
            let mut rng = rand::thread_rng();
            loop {
                let sleep_ms = rng.gen_range(0..1000);
                thread::sleep(Duration::from_millis(sleep_ms));

                lock.write(|val| {
                    *val += 1;
                    println!(
                        "Thread {} incremented value to {} after sleeping for {}ms",
                        i, *val, sleep_ms
                    );
                });
            }
        }));
    }

    // Spawn reader thread
    let lock = Arc::clone(&lock);
    handles.push(thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(1));
        lock.read(|val| {
            println!("Reader: current value is {}", *val);
        });
    }));

    // Wait for all threads
    for handle in handles {
        handle.join().unwrap();
    }
}
