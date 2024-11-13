use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{sync::Arc, thread, time::Duration};

pub struct RwSpinLock<T> {
    // Uses two counters:
    // readers: number of active readers
    // writer: 0 = no writer, 1 = writer active
    readers: AtomicUsize,
    writer: AtomicUsize,
    data: UnsafeCell<T>,
}

unsafe impl<T: Send> Send for RwSpinLock<T> {}
unsafe impl<T: Send> Sync for RwSpinLock<T> {}

impl<T> RwSpinLock<T> {
    pub fn new(data: T) -> Self {
        Self {
            readers: AtomicUsize::new(0),
            writer: AtomicUsize::new(0),
            data: UnsafeCell::new(data),
        }
    }

    pub fn read<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&T) -> R,
    {
        // Try to acquire read lock
        loop {
            // Check if there's no active writer
            if self.writer.load(Ordering::Relaxed) == 0 {
                // Increment reader count
                self.readers.fetch_add(1, Ordering::Acquire);
                // Double check writer hasn't acquired lock
                if self.writer.load(Ordering::Relaxed) == 0 {
                    break;
                }
                // Writer got in, undo reader increment
                self.readers.fetch_sub(1, Ordering::Release);
            }
            std::hint::spin_loop();
        }

        // Execute closure with immutable data
        let result = f(unsafe { &*self.data.get() });

        // Release read lock
        self.readers.fetch_sub(1, Ordering::Release);

        result
    }

    pub fn write<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut T) -> R,
    {
        // Try to acquire write lock
        loop {
            // Try to set writer flag if no writer active
            if self
                .writer
                .compare_exchange(0, 1, Ordering::Acquire, Ordering::Relaxed)
                .is_ok()
            {
                // Wait for all readers to finish
                while self.readers.load(Ordering::Relaxed) != 0 {
                    std::hint::spin_loop();
                }
                break;
            }
            std::hint::spin_loop();
        }

        // Execute closure with mutable data
        let result = f(unsafe { &mut *self.data.get() });

        // Release write lock
        self.writer.store(0, Ordering::Release);

        result
    }
}

fn main() {
    let lock = Arc::new(RwSpinLock::new(0i32));
    lock.write(|val| *val += 1);

    let mut handles = Vec::new();
    (0..10).for_each(|_| {
        let lock = Arc::clone(&lock);
        handles.push(std::thread::spawn(move || loop {
            lock.read(|val| {
                println!("Value: {}", *val);
                thread::sleep(Duration::from_secs(1));
            });
        }));
    });

    handles
        .into_iter()
        .for_each(|handle| handle.join().unwrap());
}
