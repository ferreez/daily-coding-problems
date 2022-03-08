/*
Problem 6
This problem was asked by Google.
An XOR linked list is a more memory efficient doubly linked list. 
Instead of each node holding next and prev fields, it holds a field named both, which is an XOR of the next node and the previous node. 
Implement an XOR linked list; it has an add(element) which adds the element to the end, and a get(index) which returns the node at index.

If using a language that has no pointers (such as Python), you can assume you have access to get_pointer and dereference_pointer functions that converts between nodes and memory addresses.
*/
pub struct XorLinkedList {
    element: i32,
    both: u32,
}

impl XorLinkedList {
    pub fn add(&mut self, element: i32) {
        self.element = element;
        self.both = self.both
    }

    pub fn get(&mut self, index: u32) {

    }
}

fn main() {
    println!("Hello, world!");
}
