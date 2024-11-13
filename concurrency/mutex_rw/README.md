# 🔒 RwSpinLock Implementation from Scratch

This project implements a reader-writer spin lock synchronization primitive from scratch in Rust. A reader-writer lock allows multiple concurrent readers but only one writer at a time, providing efficient shared access while maintaining thread safety.

## 🎯 Learning Goals

- Understanding reader-writer lock internals and synchronization primitives
- Working with atomic operations in Rust
- Managing thread safety and shared state
- Implementing proper read/write locking mechanisms

## 💡 Key Features

- Custom RwSpinLock implementation using atomic operations
- Multiple concurrent readers support
- Exclusive writer access
- Lock-free reader synchronization
- Spin-waiting mechanism for efficiency

## 🚀 Implementation Details

The implementation includes:

1. Atomic counter-based state tracking:
   - 0 = unlocked
   - 1 = write locked
   - n > 1 = (n-1) read locks
2. Safe concurrent read access
3. Exclusive write access
4. Memory ordering guarantees
5. Spin-loop based waiting strategy
