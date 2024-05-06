use std::{cell::RefCell, fmt, rc::Rc};

#[derive(Clone)]
struct Node<T: Clone> {
    data: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Rc<RefCell<Node<T>>>>,
}

struct DoublyLinkedList<T: Clone> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Clone> DoublyLinkedList<T> {
    fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
        }
    }

    fn push_back(&mut self, data: T) {
        let node = Rc::new(RefCell::new(Node {
            data,
            next: None,
            prev: None,
        }));

        if self.head.is_none() {
            self.head = Some(node.clone());
            self.tail = Some(node);
        } else {
            node.borrow_mut().prev.clone_from(&self.tail);
            if let Some(ref mut tail) = self.tail {
                tail.borrow_mut().next = Some(node.clone());
            }
            self.tail = Some(node);
        }
    }
}

impl<T: Clone> fmt::Display for DoublyLinkedList<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut current = self.head.clone();
        while let Some(node) = current {
            let n = node.borrow();
            write!(f, " data: {}", n.data)?;
            write!(f, " @{:p} ", &n.data)?;
            current = n.next.clone();
            if current.is_some() {
                write!(f, "<--->")?;
            }
        }

        write!(f, "\n")?;

        let mut current = self.tail.clone();
        while let Some(node) = current {
            let n = node.borrow();
            write!(f, " data: {}", n.data)?;
            write!(f, " @{:p} ", &n.data)?;
            current = n.prev.clone();
            if current.is_some() {
                write!(f, "<--->")?;
            }
        }

        Ok(())
    }
}

fn main() {
    let mut list = DoublyLinkedList::new();

    list.push_back(1);
    list.push_back(2);
    list.push_back(3);

    println!("{}", list); // 1<--->2<--->3
}
