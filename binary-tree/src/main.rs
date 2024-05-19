use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

struct TreeNode<T: Clone> {
    element: T,
    left: Option<Rc<RefCell<TreeNode<T>>>>,
    right: Option<Rc<RefCell<TreeNode<T>>>>,
}

struct BinarySearchTree<T: Clone> {
    root: Option<Rc<RefCell<TreeNode<T>>>>,
}

impl<T: Copy + Ord> BinarySearchTree<T> {
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    fn bst(&mut self, arr: &Vec<T>, start: usize, end: usize) -> Option<Rc<RefCell<TreeNode<T>>>> {
        if start == end {
            self.root = Some(Rc::new(RefCell::new(TreeNode {
                element: arr[start],
                left: None,
                right: None,
            })));
        }

        let mid = (start + end) / 2;
        let mut left = None;
        if start < mid {
            left = self.bst(arr, start, mid - 1);
        }
        let mut right = None;
        if end > mid {
            right = self.bst(arr, mid + 1, end);
        }

        self.root = Some(Rc::new(RefCell::new(TreeNode {
            element: arr[mid],
            left,
            right,
        })));

        self.root.clone()
    }

    fn insert(&mut self, value: T) {
        let mut iterator = self.root.clone();
        while let Some(node) = iterator {
            if node.borrow().left.is_none() && node.borrow().element > value {
                node.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode {
                    element: value,
                    left: None,
                    right: None,
                })))
            } else if node.borrow().right.is_none() && node.borrow().element < value {
                node.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode {
                    element: value,
                    left: None,
                    right: None,
                })))
            }
            iterator = if node.borrow().element > value {
                node.borrow().left.clone()
            } else {
                node.borrow().right.clone()
            };
        }
    }

    fn minimum_node(&self) -> Option<Rc<RefCell<TreeNode<T>>>> {
        let mut iterator = self.root.clone();
        while let Some(node) = iterator {
            if node.borrow().left.is_none() {
                return Some(node);
            }
            iterator = node.borrow().left.clone();
        }
        None
    }

    fn find_value(&self, value: T) -> Option<Rc<RefCell<TreeNode<T>>>> {
        let mut iterator = self.root.clone();
        while let Some(node) = iterator {
            if node.borrow().element == value {
                return Some(node);
            }

            iterator = if node.borrow().element > value {
                node.borrow().left.clone()
            } else {
                node.borrow().right.clone()
            };
        }
        None
    }

    fn inorder_successor(&self, value: T) -> Option<Rc<RefCell<TreeNode<T>>>> {
        let node = match self.find_value(value) {
            Some(target_node) => target_node,
            None => return None,
        };

        if let Some(ref right_node) = node.borrow().right {
            return right_node.borrow().minimum_node();
        }

        let mut succ = None;
        let mut iterator = self.root.clone();
        while let Some(cur_node) = iterator.clone() {
            if node.borrow().element < cur_node.borrow().element {
                succ = Some(cur_node.clone());
                iterator = cur_node.borrow().left.clone();
            } else if node.borrow().element > cur_node.borrow().element {
                iterator = cur_node.borrow().right.clone();
            } else {
                break;
            }
        }
        succ
    }

    fn delete(&mut self, value: T) {
        if let Some(root) = self.root.clone() {
            root.borrow_mut().delete(value);
        }
    }
}

impl<T: Clone + std::cmp::PartialOrd> TreeNode<T> {
    fn minimum_node(&self) -> Option<Rc<RefCell<TreeNode<T>>>> {
        if let Some(ref left) = self.left {
            left.borrow().minimum_node()
        } else {
            Some(Rc::new(RefCell::new(TreeNode {
                element: self.element.clone(),
                left: self.left.clone(),
                right: self.right.clone(),
            })))
        }
    }

    fn delete(&mut self, value: T) -> Option<Rc<RefCell<TreeNode<T>>>> {
        if self.element < value {
            if let Some(right_node) = self.right.clone() {
                self.right = right_node.borrow_mut().delete(value);
            }
        } else if self.element > value {
            if let Some(left_node) = self.left.clone() {
                self.left = left_node.borrow_mut().delete(value);
            }
        } else {
            if self.left.is_none() {
                return self.right.clone();
            } else if self.right.is_none() {
                return self.left.clone();
            }
            if let Some(right_node) = self.right.clone() {
                let temp = right_node.borrow().minimum_node();
                if let Some(temp_data) = temp {
                    self.element = temp_data.borrow().element.clone();
                    self.right = right_node
                        .borrow_mut()
                        .delete(temp_data.borrow().element.clone());
                }
            }
        }
        Some(Rc::new(RefCell::new(TreeNode {
            element: self.element.clone(),
            left: self.left.clone(),
            right: self.right.clone(),
        })))
    }
}

impl<T: Copy> fmt::Display for BinarySearchTree<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f)?;
        //         use std::collections::VecDeque;
        //
        //         let mut queue = VecDeque::new();
        //         if let Some(root) = self.root.clone() {
        //             queue.push_back(root);
        //         }
        //
        //         while let Some(cur_node) = queue.pop_front() {
        //             write!(
        //                 f,
        //                 " {} @{:p} ",
        //                 cur_node.borrow().element,
        //                 &cur_node.borrow().element
        //             )?;
        //
        //             if let Some(node_left) = cur_node.borrow().left.clone() {
        //                 queue.push_back(node_left)
        //             }
        //             if let Some(node_right) = cur_node.borrow().right.clone() {
        //                 queue.push_back(node_right)
        //             }
        //         }

        // inorder
        if let Some(ref node) = self.root {
            node.borrow().fmt(f)?;
        }
        Ok(())
    }
}

impl<T: Copy> fmt::Display for TreeNode<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref left) = self.left {
            left.borrow().fmt(f)?;
        }
        write!(f, "{} @{:p} -> ", self.element, &self.element)?;
        if let Some(ref right) = self.right {
            right.borrow().fmt(f)?;
        }
        Ok(())
    }
}

fn main() {
    //                 5
    //              ↙︎     ↘︎
    //            2          8
    //          ↙︎  ↘︎      ↙︎    ↘︎
    //         1   3     6      9
    //              ↘︎     ↘︎       ↘︎
    //              4     7       10
    let mut bst = BinarySearchTree::new();

    let arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    bst.bst(&arr, 0, arr.len() - 1);
    println!("{}", bst); // 1 -> 2 -> 3 -> 4 -> 5 -> 6 -> 7 -> 8 -> 9 -> 10

    bst.delete(2);
    println!("{}", bst); // 1 -> 3 -> 4 -> 5 -> 6 -> 7 -> 8 -> 9 -> 10

    bst.delete(4);
    println!("{}", bst); // 1 -> 3 -> 5 -> 6 -> 7 -> 8 -> 9 -> 10

    bst.insert(11);
    println!("{}", bst); // 1 -> 3 -> 5 -> 6 -> 7 -> 8 -> 9 -> 10 -> 11

    bst.delete(20);
    println!("{}", bst); // 1 -> 3 -> 5 -> 6 -> 7 -> 8 -> 9 -> 10 -> 11
}
