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
        // inorder
        match *self {
            BinaryTree::NonEmpty(ref node) => {
                node.left.fmt(f)?;
                write!(f, "{} -> ", node.element)?;
                node.right.fmt(f)?;
            }
            BinaryTree::Empty => {}
        }
        Ok(())
    }
}

fn main() {
    let arr = vec![10, 1, 5, 8, 9, 6];
    let mut bst = BinarySearchTree::new(arr);
    println!("{}", bst);

    bst.insert(0);
    bst.insert(4);

    println!("{}", bst);
}
