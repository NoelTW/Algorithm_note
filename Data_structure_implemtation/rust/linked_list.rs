struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node { value, next: None }
    }
}

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    tail: Option<*mut Node<T>>,
    length: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn append(&mut self, value: T) {
        let new_node = Box::new(Node::new(value));
        match self.tail {
            Some(tail_ptr) => unsafe {
                (*tail_ptr).next = Some(new_node);
            },
            None => self.head = Some(new_node),
        }
        self.tail = Some(Box::into_raw(new_node) as *mut _);
        self.length += 1;
    }

    pub fn prepend(&mut self, value: T) {
        let new_node = Box::new(Node::new(value));
        match self.head.take() {
            Some(old_head) => {
                new_node.next = Some(old_head);
                self.head = Some(new_node);
            }
            None => {
                self.head = Some(new_node);
                self.tail = Some(&mut **self.head.as_mut().unwrap() as *mut _);
            }
        }
        self.length += 1;
    }

    fn traverse(&self, index: usize) -> Option<*mut Node<T>> {
        let mut current = self.head.as_ref()?;
        let mut i = 0;
        while i < index {
            current = current.next.as_ref()?;
            i += 1;
        }
        Some(&mut **current as *mut _)
    }

    pub fn insert(&mut self, index: usize, value: T) {
        if index >= self.length {
            self.append(value);
            return;
        }
        if index == 0 {
            self.prepend(value);
            return;
        }
        let new_node = Box::new(Node::new(value));
        let prev_node_ptr = self.traverse(index - 1).unwrap();
        unsafe {
            (*new_node).next = (*prev_node_ptr).next.take();
            (*prev_node_ptr).next = Some(new_node);
        }
        self.length += 1;
    }

    pub fn remove(&mut self, index: usize) {
        if self.length == 0 || index >= self.length {
            return;
        }
        if index == 0 {
            self.head = self.head.as_mut().unwrap().next.take();
            if self.length == 1 {
                self.tail = None;
            }
            self.length -= 1;
            return;
        }
        let prev_node_ptr = self.traverse(index - 1).unwrap();
        unsafe {
            let node_to_remove = (*prev_node_ptr).next.take();
            (*prev_node_ptr).next = (*node_to_remove.unwrap()).next.take();
            if index == self.length - 1 {
                self.tail = prev_node_ptr;
            }
        }
        self.length -= 1;
    }

    pub fn reverse(&mut self) {
        let mut prev_node = None;
        let mut current_node = self.head.take();
        while let Some(mut curr) = current_node.take() {
            let next_node = curr.next.take();
            curr.next = prev_node.take();
            prev_node = Some(curr);
            current_node = next_node;
        }
        self.head = prev_node.take();
        if let Some(mut head) = self.head.as_mut() {
            while let Some(mut next_node) = head.next.take() {
                head.next = prev_node.take();
                prev_node = Some(head);
                head = next_node;
            }
            self.tail = Some(&mut **self.head.as_mut().unwrap() as *mut _);
        }
    }

    pub fn to_vec(&self) -> Vec<&T> {
        let mut result = Vec::with_capacity(self.length);
        let mut current = &self.head;
        while let Some(node) = current {
            result.push(&node.value);
            current = &node.next;
        }
        result
    }
}

fn main() {
    let mut list: LinkedList<i32> = LinkedList::new();

    // Test prepend function
    println!("Testing prepend function:");
    list.prepend(10);
    println!("List after prepending 10: {:?}", list.to_vec());

    // Test append function
    println!("\nTesting append function:");
    list.append(1);
    println!("List after appending 1: {:?}", list.to_vec());

    // Test insert function
    println!("\nTesting insert function:");
    list.insert(1, 0);
    println!("List after inserting 0 at index 1: {:?}", list.to_vec());

    // Test insert function with out of bounds index
    println!("\nTesting insert function with out of bounds index:");
    list.insert(11000, 10);
    println!("List after inserting 10 at index 11000: {:?}", list.to_vec());

    // Test reverse function
    println!("\nTesting reverse function:");
    list.reverse();
    println!("List be reversed: {:?}", list.to_vec());

    // Test remove function
    println!("\nTesting remove function:");
    list.remove(2);
    println!("List after removing element at index 2: {:?}", list.to_vec());

    // Test remove function with out of bounds index
    println!("\nTesting remove function with out of bounds index:");
    list.remove(10);
    println!("List after attempting to remove element at index 10: {:?}", list.to_vec());

    // Test remove function when list is empty
    println!("\nTesting remove function when list is empty:");
    list.remove(0);
    println!("List after attempting to remove element from empty list: {:?}", list.to_vec());

    // Test remove function on single element list
    println!("\nTesting remove function on single element list:");
    list.append(5);
    list.remove(0);
    println!("List after removing single element: {:?}", list.to_vec());
}
