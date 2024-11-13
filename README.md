# 🦀 Rust Learning Adventures!

Welcome to my exciting journey of learning Rust! This repository is my playground for exploring the powerful features and concepts of the Rust programming language through hands-on projects.

## 🚀 Projects

### 1. 📚 Student Management System

A command-line application implementing a student record management system with quicksort!

- 📂 Location: `/student_management_system`
- 🎯 Status: In Progress
- ✨ Features:
  - Custom Student type with name, age, and grade fields
  - Generic quicksort implementation for sorting students
  - Multi-level sorting by grade, age, and name
  - Proper handling of floating-point comparisons

### 2. 🔒 Mutex Implementation

A from-scratch implementation of mutex synchronization primitives!

- 📂 Location: `/concurrency/mutex`
- 🎯 Status: In Progress
- ✨ Features:
  - Custom mutex using atomic operations
  - RAII-style lock guards (pending)
  - Deadlock prevention (pending)
  - Thread parking/unparking for efficiency (I'm not sure if it's implemented correctly)

### 3. 🔒 Reader-Preference Read-Write Mutex Implementation

A specialized mutex allowing multiple concurrent readers!

- 📂 Location: `/concurrency/mutex_rw`
- 🎯 Status: In Progress
- ✨ Features:
  - Multiple simultaneous read access
  - Exclusive write access
  - RAII-style lock guards (pending)
  - Atomic-based synchronization

## 🏁 Getting Started

### 🔧 Prerequisites

- Rust (latest stable version)
- Cargo (comes bundled with Rust)

### 🎮 Building and Running

Each project can be built and run independently:

1. Navigate to a project directory:

   ```bash
   cd student_management_system  # or any other project directory
   ```

2. Build and run the project:
   ```bash
   cargo run
   ```

The project will compile and execute, displaying its output in the terminal.
