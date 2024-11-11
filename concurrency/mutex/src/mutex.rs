use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicBool, Ordering};

const LOCKED: bool = true;
const UNLOCKED: bool = false;

pub struct Mutex<T> {
    data: UnsafeCell<T>,
    locked: AtomicBool,
}

unsafe impl<T> Sync for Mutex<T> where T: Send {}

impl<T> Mutex<T> {
    pub fn new(data: T) -> Self {
        Self {
            data: UnsafeCell::new(data),
            locked: AtomicBool::new(UNLOCKED),
        }
    }

    #[allow(dead_code)]
    pub fn wrong_lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        while self.locked.load(Ordering::Relaxed) == LOCKED {}
        self.locked.store(LOCKED, Ordering::Relaxed);
        let res = f(unsafe { &mut *self.data.get() });
        self.locked.store(UNLOCKED, Ordering::Relaxed);
        res
    }

    #[allow(dead_code)]
    pub fn still_wrong_lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        while self.locked.load(Ordering::Acquire) == LOCKED {}
        self.locked.store(LOCKED, Ordering::Release);
        let res = f(unsafe { &mut *self.data.get() });
        self.locked.store(UNLOCKED, Ordering::Relaxed);
        res
    }

    #[allow(dead_code)]
    pub fn spin_lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        while self
            .locked
            .compare_exchange(UNLOCKED, LOCKED, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {}
        let res = f(unsafe { &mut *self.data.get() });
        self.locked.store(UNLOCKED, Ordering::Release);
        res
    }

    pub fn spin_lock_with_hint<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        while self
            .locked
            .compare_exchange(UNLOCKED, LOCKED, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            std::hint::spin_loop();
        }
        let res = f(unsafe { &mut *self.data.get() });
        self.locked.store(UNLOCKED, Ordering::Release);
        res
    }
}
