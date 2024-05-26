use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

struct Node<T: Clone> {
    data: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

struct Stack<T: Clone> {
    head: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Clone> Stack<T> {
    fn new() -> Self {
        Stack { head: None }
    }

    fn push(&mut self, value: T) {
        let temp = self.head.clone();
        self.head = Some(Rc::new(RefCell::new(Node {
            data: value,
            next: None,
        })));

        if let Some(head) = self.head.clone() {
            head.borrow_mut().next = temp
        }
    }

    fn pop(&mut self) -> Option<T> {
        match self.head.clone() {
            Some(head) => {
                let temp = head.borrow();
                self.head.clone_from(&head.borrow().next);
                Some(temp.data.clone())
            }
            None => None,
        }
    }
}

impl<T: Clone> fmt::Display for Stack<T>
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
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    println!("{}", stack);
    if let Some(value) = stack.pop() {
        println!(" pop: {}\n", value);
    }
    println!("{}", stack);
}
