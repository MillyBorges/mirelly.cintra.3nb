
pub struct Node<T> {
    elem: T,
    next: Option<Box<Node<T>>>,
}

pub struct Queue<T> {
    head: Option<Box<Node<T>>>,
    tail: Option<*mut Node<T>>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            head: None,
            tail: None,
        }
    }
}    
    impl<T> Queue<T> {
        pub fn enqueue(&mut self, elem: T) {
            let mut new_node = Box::new(Node {
                elem,
                next: None,
            });
    
            let new_node_ptr: *mut Node<T> = &mut *new_node;

            if let Some(tail_ptr) = self.tail {
                unsafe {
                    (*tail_ptr).next = Some(new_node);
                }
            } else {
                self.head = Some(new_node);
            }
            self.tail = Some(new_node_ptr);
            if self.head.is_none() {
                self.head = Some(unsafe { Box::from_raw(new_node_ptr) });
            }
        }
    }

    impl<T> Queue<T> {
        pub fn dequeue(&mut self) -> Option<T> {
            self.head.take().map(|boxed_node| {
                self.head = boxed_node.next;
    
                if self.head.is_none() {
                    self.tail = None;
                }
                boxed_node.elem
            })
        }
    }

impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        while let Some(_) = self.dequeue() {}
    }
}

// src/lib.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enqueue_dequeue() {
        let mut queue = Queue::new();

        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));

        assert_eq!(queue.dequeue(), None);
    }
}