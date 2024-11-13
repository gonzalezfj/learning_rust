use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{sync::Arc, thread, time::Duration};

pub struct SpinLock<T> {
    locked: AtomicUsize,
    data: UnsafeCell<T>,
}

unsafe impl<T: Send> Send for SpinLock<T> {}
unsafe impl<T: Send> Sync for SpinLock<T> {}

impl<T> SpinLock<T> {
    pub fn new(data: T) -> Self {
        Self {
            locked: AtomicUsize::new(0),
            data: UnsafeCell::new(data),
        }
    }

    pub fn lock<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut T) -> R,
    {
        // Try to acquire lock
        while self
            .locked
            .compare_exchange(0, 1, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            std::hint::spin_loop();
        }

        // Execute closure with mutable data
        let result = f(unsafe { &mut *self.data.get() });

        // Release lock
        self.locked.store(0, Ordering::Release);

        result
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::time::Instant;

    #[test]
    fn test_basic_read_write() {
        let lock = RwSpinLock::new(0i32);

        // Test write
        lock.write(|val| *val = 42);

        // Test read
        let value = lock.read(|val| *val);
        assert_eq!(value, 42);
    }

    #[test]
    fn test_multiple_readers() {
        let lock = Arc::new(RwSpinLock::new(42i32));
        let mut handles = Vec::new();

        // Spawn multiple reader threads
        for _ in 0..5 {
            let lock_clone = Arc::clone(&lock);
            handles.push(thread::spawn(move || {
                let val = lock_clone.read(|val| *val);
                assert_eq!(val, 42);
            }));
        }

        // All readers should complete without deadlock
        for handle in handles {
            handle.join().unwrap();
        }
    }

    #[test]
    fn test_writer_blocks_readers() {
        let lock = Arc::new(RwSpinLock::new(0i32));
        let lock_clone = Arc::clone(&lock);

        // Start a long write operation
        let write_handle = thread::spawn(move || {
            lock_clone.write(|val| {
                *val += 1;
                thread::sleep(Duration::from_millis(100));
                *val
            })
        });

        // Try to read immediately after - should still see 0
        let read_result = lock.read(|val| *val);
        assert_eq!(read_result, 0);

        // Write should complete with 1
        assert_eq!(write_handle.join().unwrap(), 1);

        // Reading after write completes should see 1
        let final_read = lock.read(|val| *val);
        assert_eq!(final_read, 1);
    }

    #[test]
    fn test_concurrent_modifications() {
        let lock = Arc::new(RwSpinLock::new(0i32));
        let mut handles = Vec::new();

        // Spawn multiple writer threads
        for _ in 0..10 {
            let lock_clone = Arc::clone(&lock);
            handles.push(thread::spawn(move || lock_clone.write(|val| *val += 1)));
        }

        // Wait for all writers
        for handle in handles {
            handle.join().unwrap();
        }

        // Verify final value
        let final_value = lock.read(|val| *val);
        assert_eq!(final_value, 10);
    }

    #[test]
    fn benchmark_rw_vs_normal_spin() {
        // Create locks
        let rw_lock = Arc::new(RwSpinLock::new(0i32));
        let normal_lock = Arc::new(SpinLock::new(0i32));
        let running = Arc::new(AtomicBool::new(true));

        const NUM_READERS: usize = 8;
        const NUM_WRITERS: usize = 2;
        const TEST_DURATION_MS: u64 = 1000;

        // RW Lock benchmark
        let mut rw_handles = Vec::new();
        let start_time = Instant::now();
        let running_rw = Arc::clone(&running);

        // Spawn reader threads
        for _ in 0..NUM_READERS {
            let lock = Arc::clone(&rw_lock);
            let running = Arc::clone(&running_rw);
            rw_handles.push(thread::spawn(move || {
                let mut reads = 0;
                while running.load(Ordering::Relaxed) {
                    lock.read(|_| {});
                    reads += 1;
                }
                reads
            }));
        }

        // Spawn writer threads
        for _ in 0..NUM_WRITERS {
            let lock = Arc::clone(&rw_lock);
            let running = Arc::clone(&running_rw);
            rw_handles.push(thread::spawn(move || {
                let mut writes = 0;
                while running.load(Ordering::Relaxed) {
                    lock.write(|val| *val += 1);
                    writes += 1;
                }
                writes
            }));
        }

        thread::sleep(Duration::from_millis(TEST_DURATION_MS));
        running.store(false, Ordering::Relaxed);

        let rw_total_ops: usize = rw_handles.into_iter().map(|h| h.join().unwrap()).sum();
        let rw_ops_per_sec = rw_total_ops as f64 / start_time.elapsed().as_secs_f64();

        // Normal SpinLock benchmark
        running.store(true, Ordering::Relaxed);
        let mut normal_handles = Vec::new();
        let start_time = Instant::now();

        // Spawn threads that both read and write
        for _ in 0..(NUM_READERS + NUM_WRITERS) {
            let lock = Arc::clone(&normal_lock);
            let running = Arc::clone(&running);
            normal_handles.push(thread::spawn(move || {
                let mut ops = 0;
                while running.load(Ordering::Relaxed) {
                    lock.lock(|_| {});
                    ops += 1;
                }
                ops
            }));
        }

        thread::sleep(Duration::from_millis(TEST_DURATION_MS));
        running.store(false, Ordering::Relaxed);

        let normal_total_ops: usize = normal_handles.into_iter().map(|h| h.join().unwrap()).sum();
        let normal_ops_per_sec = normal_total_ops as f64 / start_time.elapsed().as_secs_f64();

        println!("RW SpinLock ops/sec: {:.2}", rw_ops_per_sec);
        println!("Normal SpinLock ops/sec: {:.2}", normal_ops_per_sec);

        // RW lock should be faster due to concurrent reads
        assert!(rw_ops_per_sec > normal_ops_per_sec);
    }
}
