use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

struct Node<T: Clone> {
    data: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

struct Queue<T: Clone> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Clone> Queue<T> {
    fn new() -> Self {
        Queue {
            head: None,
            tail: None,
        }
    }

    fn enqueue(&mut self, value: T) {
        if self.head.is_none() {
            self.head = Some(Rc::new(RefCell::new(Node {
                data: value,
                next: None,
            })));
            self.tail.clone_from(&self.head.clone());
        } else if let Some(tail) = self.tail.clone() {
            tail.borrow_mut()
                .next
                .clone_from(&Some(Rc::new(RefCell::new(Node {
                    data: value,
                    next: None,
                }))));
            self.tail.clone_from(&tail.borrow().next.clone());
        }
    }

    fn dequeue(&mut self) -> Option<T> {
        match self.head.clone() {
            Some(head) => {
                let temp = head.borrow();
                self.head.clone_from(&head.borrow().next.clone());
                if self.head.is_none() {
                    self.tail = None;
                }
                Some(temp.data.clone())
            }
            None => None,
        }
    }
}

impl<T: Clone> fmt::Display for Queue<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut current = self.head.clone();
        while let Some(node) = current.clone() {
            let n = node.borrow();
            write!(f, " data: {}", n.data)?;
            write!(f, " @{:p} ", &n.data)?;
            current.clone_from(&n.next);
            if current.is_some() {
                write!(f, "<--->")?;
            }
        }

        writeln!(f)?;

        Ok(())
    }
}

fn main() {
    let mut queue = Queue::new();
    queue.enqueue(1);
    queue.enqueue(2);
    queue.enqueue(3);

    println!("{}", queue);
    if let Some(value) = queue.dequeue() {
        println!(" dequeue: {}\n", value);
    }
    println!("{}", queue);
}
