use std::cell::RefCell;
use std::{fmt, rc::Rc};

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

    fn push_front(&mut self, data: T) {
        let node = Rc::new(RefCell::new(Node {
            data,
            next: None,
            prev: None,
        }));

        if self.head.is_none() {
            self.head = Some(node.clone());
            self.tail = Some(node);
        } else {
            node.borrow_mut().next.clone_from(&self.head);
            if let Some(ref mut head) = self.head {
                head.borrow_mut().prev = Some(node.clone());
            }
            self.head = Some(node);
        }
    }

    fn pop_front(&mut self) -> Option<Node<T>> {
        match self.head.clone() {
            Some(head) => {
                let head_data = head.borrow().clone();
                if let Some(next) = head.borrow().next.clone() {
                    next.borrow_mut().prev = None;
                    self.head.clone_from(&head.borrow().next.clone());
                }

                Some(head_data)
            }
            None => None,
        }
    }

    fn pop_back(&mut self) -> Option<Node<T>> {
        match self.tail.clone() {
            Some(tail) => {
                let tail_data = tail.borrow().clone();
                if let Some(prev) = tail.borrow().prev.clone() {
                    prev.borrow_mut().next = None;
                    self.tail.clone_from(&tail.borrow().prev.clone());
                }

                Some(tail_data)
            }
            None => None,
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

        writeln!(f)?;

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
    println!("{}\n", list); // 1<--->2<--->3

    list.push_front(4);
    list.push_front(5);
    println!("{}\n", list); // 5<--->4<--->1<--->2<--->3

    if let Some(node) = list.pop_front() {
        println!(" data: {}", node.data);
    }
    println!("{}\n", list); // 4<--->1<--->2<--->3

    if let Some(node) = list.pop_back() {
        println!(" data: {}", node.data);
    }
    println!("{}", list); // 4<--->1<--->2
}
