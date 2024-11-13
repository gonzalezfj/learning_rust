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

The implementation uses two atomic counters:

- `readers`: Tracks number of active readers
- `writer`: 0 = no writer, 1 = writer active

Key aspects:

1. Reader acquisition:
   - Check no active writer
   - Increment reader count atomically
   - Double-check no writer sneaked in
2. Writer acquisition:
   - Set writer flag atomically
   - Wait for all readers to finish
3. Memory ordering guarantees via Acquire/Release semantics
4. Spin-loop based waiting for better CPU efficiency

## 🧪 Testing

Includes comprehensive tests for:

- Basic read/write functionality
- Multiple concurrent readers
- Writer blocking readers
- Concurrent modifications
- Performance benchmarks vs regular spinlock
