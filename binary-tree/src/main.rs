use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

struct TreeNode<T> {
    element: T,
    left: Option<Rc<RefCell<TreeNode<T>>>>,
    right: Option<Rc<RefCell<TreeNode<T>>>>,
}

struct BinarySearchTree<T> {
    root: Option<Rc<RefCell<TreeNode<T>>>>,
}

impl<T: Copy + Ord> BinarySearchTree<T> {
    fn new(mut arr: Vec<T>) -> Self {
        arr.sort();
        BinarySearchTree {
            root: Self::sorted_bst(&arr, 0, arr.len() - 1),
        }
    }

    fn sorted_bst(arr: &Vec<T>, start: usize, end: usize) -> Option<Rc<RefCell<TreeNode<T>>>> {
        if start == end {
            return Some(Rc::new(RefCell::new(TreeNode {
                element: arr[start],
                left: None,
                right: None,
            })));
        }

        let mid = (start + end) / 2;
        let mut left = None;
        if start < mid {
            left = Self::sorted_bst(arr, start, mid - 1);
        }
        let mut right = None;
        if end > mid {
            right = Self::sorted_bst(arr, mid + 1, end);
        }

        Some(Rc::new(RefCell::new(TreeNode {
            element: arr[mid],
            left,
            right,
        })))
    }

    fn insert(&mut self, value: T) {
        let mut iterator = self.root.clone();
        while let Some(node) = iterator {
            if node.borrow().left.clone().is_none() && node.borrow().element > value {
                node.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode {
                    element: value,
                    left: None,
                    right: None,
                })))
            } else if node.borrow().right.clone().is_none() && node.borrow().element < value {
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

    fn search(&self, value: T) -> Option<Rc<RefCell<TreeNode<T>>>> {
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
}

impl<T: Copy> fmt::Display for BinarySearchTree<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::collections::VecDeque;

        let mut queue = VecDeque::new();
        if let Some(root) = self.root.clone() {
            queue.push_back(root);
        }

        while let Some(cur_node) = queue.pop_front() {
            write!(
                f,
                " {} @{:p} ",
                cur_node.borrow().element,
                &cur_node.borrow().element
            )?;

            if let Some(node_left) = cur_node.borrow().left.clone() {
                queue.push_back(node_left)
            }
            if let Some(node_right) = cur_node.borrow().right.clone() {
                queue.push_back(node_right)
            }
        }

        // inorder
        // match *self {
        //     BinaryTree::NonEmpty(ref node) => {
        //         node.left.fmt(f)?;
        //         write!(f, "{} @{:p}-> ", node.element, &node.element)?;
        //         node.right.fmt(f)?;
        //     }
        //     BinaryTree::Empty => {}
        // }
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
    let arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut bst = BinarySearchTree::new(arr);
    println!("{}", bst);

    if let Some(node) = bst.search(8) {
        println!("\nbinary search: {}\n", node.borrow().element);
    }

    bst.insert(0);
    bst.insert(11);

    println!("{}", bst);
}
