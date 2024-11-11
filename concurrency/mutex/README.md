# 🔒 Mutex Implementation from Scratch

This project implements a basic mutex (mutual exclusion) synchronization primitive from scratch in Rust. A mutex ensures that only one thread can access a shared resource at a time, preventing data races and ensuring thread safety.

## 🎯 Learning Goals

- Understanding mutex internals and synchronization primitives
- Working with atomic operations in Rust
- Managing thread safety and shared state
- Implementing proper locking and unlocking mechanisms

## 💡 Key Features

- Custom mutex implementation using atomic operations
- Safe thread synchronization
- RAII-style lock guards
- Deadlock prevention mechanisms

## 🚀 Implementation Details

The implementation includes:

1. Atomic-based locking mechanism
2. Lock guard for RAII-style resource management
3. Poisoning detection for panic safety
4. Thread parking/unparking for efficient waiting

## 📚 Usage Example
