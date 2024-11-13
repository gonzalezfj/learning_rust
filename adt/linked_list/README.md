# 🔗 Linked List Implementation in Rust

This project implements a singly linked list data structure in Rust, demonstrating fundamental concepts of data structures and memory management in Rust.

## 🎯 Learning Goals

- Understanding linked list data structures
- Working with Box<T> for heap allocation
- Implementing generic types in Rust
- Managing ownership and borrowing
- Working with Option<T> for null safety

## 💡 Key Features

- Generic linked list implementation
- Basic operations:
  - Push front
  - Pop front
  - Check if empty
  - Print list contents
- Memory safety through Rust's ownership system
- Debug trait implementation for easy printing

## 🚀 Implementation Details

The implementation uses two main structures:

- `Node<T>`: Represents a single node containing:
  - `data`: The stored value of type T
  - `next`: Option<Box<Node<T>>> for the next node
- `LinkedList<T>`: The list container with:
  - `head`: Option<Box<Node<T>>> for the first node

Key aspects:

1. Generic type T allows storing any type that implements Debug
2. Box<T> provides heap allocation for nodes
3. Option<T> handles null cases safely
4. Debug trait enables printing of list contents

## 🧪 Testing

The implementation includes example usage demonstrating:

- List creation
- Adding elements
- Removing elements
- Checking empty state
- Printing list contents

## 🔜 Future Enhancements

Planned improvements include:

- Push/pop operations for back of list
- Insert at arbitrary positions
- Remove at arbitrary positions
- Iterator implementation
- Additional utility methods
