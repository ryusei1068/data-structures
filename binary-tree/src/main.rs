use std::fmt;

struct TreeNode<T: Copy> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

enum BinaryTree<T: Copy> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

struct BinarySearchTree<T: Copy> {
    root: BinaryTree<T>,
}

impl<T: Ord + Copy> BinarySearchTree<T> {
    fn new(mut arr: Vec<T>) -> Self {
        arr.sort();
        BinarySearchTree {
            root: Self::sorted_bst(&arr, 0, arr.len() - 1),
        }
    }

    fn sorted_bst(arr: &Vec<T>, start: usize, end: usize) -> BinaryTree<T> {
        if start == end {
            return BinaryTree::NonEmpty(Box::new(TreeNode {
                element: arr[start],
                left: BinaryTree::Empty,
                right: BinaryTree::Empty,
            }));
        }

        let mid = (start + end) / 2;
        let mut left = BinaryTree::Empty;
        if start < mid {
            left = Self::sorted_bst(arr, start, mid - 1);
        }
        let mut right = BinaryTree::Empty;
        if end > mid {
            right = Self::sorted_bst(arr, mid + 1, end);
        }

        BinaryTree::NonEmpty(Box::new(TreeNode {
            element: arr[mid],
            left,
            right,
        }))
    }

    fn insert(&mut self, value: T) {
        self.root.add(value);
    }

    fn search(&self, key: T) -> Option<&TreeNode<T>> {
        self.root.search(key)
    }
}

impl<T: Ord + Copy> BinaryTree<T> {
    fn add(&mut self, value: T) {
        match *self {
            BinaryTree::Empty => {
                *self = BinaryTree::NonEmpty(Box::new(TreeNode {
                    element: value,
                    left: BinaryTree::Empty,
                    right: BinaryTree::Empty,
                }))
            }
            BinaryTree::NonEmpty(ref mut node) => {
                if value <= node.element {
                    node.left.add(value)
                } else {
                    node.right.add(value)
                }
            }
        }
    }

    fn search(&self, value: T) -> Option<&TreeNode<T>> {
        use std::cmp::Ordering;

        match *self {
            BinaryTree::NonEmpty(ref node) => match node.element.cmp(&value) {
                Ordering::Greater => node.left.search(value),
                Ordering::Less => node.right.search(value),
                Ordering::Equal => Some(node),
            },
            BinaryTree::Empty => None,
        }
    }
}

impl<T: Copy> fmt::Display for BinarySearchTree<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.root.fmt(f)?;
        Ok(())
    }
}

impl<T: Copy> fmt::Display for BinaryTree<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::collections::VecDeque;

        let mut queue = VecDeque::new();
        queue.push_back(self);

        while let Some(cur_node) = queue.pop_front() {
            match *cur_node {
                BinaryTree::NonEmpty(ref cur_node) => {
                    write!(f, " {} @{:p} ", cur_node.element, &cur_node.element)?;
                    match cur_node.left {
                        BinaryTree::NonEmpty(_) => {
                            queue.push_back(&cur_node.left);
                        }
                        BinaryTree::Empty => {}
                    }
                    match cur_node.right {
                        BinaryTree::NonEmpty(_) => {
                            queue.push_back(&cur_node.right);
                        }
                        BinaryTree::Empty => {}
                    }
                }
                BinaryTree::Empty => {}
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
        println!("\nbinary search: {}\n", node.element);
    }

    bst.insert(0);
    bst.insert(11);

    println!("{}", bst);
}
