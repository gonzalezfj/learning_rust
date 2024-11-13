use std::fmt::Debug;

// Node structure for the linked list
#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

// LinkedList structure
#[derive(Debug)]
struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T>
where
    T: Debug,
{
    // Create a new empty linked list
    fn new() -> Self {
        LinkedList { head: None }
    }

    // Add element to front of list
    fn push_front(&mut self, data: T) {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    // Remove and return front element
    fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }

    // Check if list is empty
    fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    // Print the list
    fn print(&self) {
        let mut current = &self.head;
        while let Some(node) = current {
            print!("{:?} -> ", node.data);
            current = &node.next;
        }
        println!("None");
    }
}

fn main() {
    // Create a new linked list
    let mut list: LinkedList<i32> = LinkedList::new();

    // Add some elements
    list.push_front(3);
    list.push_front(2);
    list.push_front(1);

    println!("Initial list:");
    list.print();

    // Pop front element
    println!("\nPopped value: {:?}", list.pop_front());

    println!("List after pop:");
    list.print();

    // Check if empty
    println!("\nIs list empty? {}", list.is_empty());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_list_is_empty() {
        let list: LinkedList<i32> = LinkedList::new();
        assert!(list.is_empty());
    }

    #[test]
    fn test_push_front() {
        let mut list = LinkedList::new();
        list.push_front(1);
        assert!(!list.is_empty());
    }

    #[test]
    fn test_pop_front() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn test_multiple_operations() {
        let mut list = LinkedList::new();
        assert!(list.is_empty());

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        assert!(!list.is_empty());

        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        assert!(!list.is_empty());

        list.push_front(4);
        assert_eq!(list.pop_front(), Some(4));
        assert_eq!(list.pop_front(), Some(1));
        assert!(list.is_empty());
    }
}
