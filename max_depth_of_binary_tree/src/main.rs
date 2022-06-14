// EASY
//
// // Given the root of a binary tree, return its maximum depth.
// A binary tree's maximum depth is the number of nodes along the longest path from the root node down to the farthest leaf node.
//
// Example 1:
//
// Input: root = [3,9,20,null,null,15,7]
// Output: 3
//
// Example 2:
//
// Input: root = [1,null,2]
// Output: 2
//
// Constraints:
//
// The number of nodes in the tree is in the range [0, 10^4].
// -100 <= Node.val <= 100

use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn depth(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if node.is_none() {
        return 0;
    }

    let lhs = &node.as_ref().unwrap().borrow().left;
    let rhs = &node.as_ref().unwrap().borrow().right;

    let left_depth = depth(lhs);
    let right_depth = depth(rhs);

    cmp::max(left_depth, right_depth) + 1
}

fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    depth(&root)
}

fn main() {
    let mut root = TreeNode::new(3);
    let lhs = TreeNode::new(9);
    let mut rhs = TreeNode::new(20);
    let lhs_20 = TreeNode::new(15);
    let rhs_20 = TreeNode::new(20);

    root.left = Some(Rc::new(RefCell::new(lhs)));
    rhs.left = Some(Rc::new(RefCell::new(lhs_20)));
    rhs.right = Some(Rc::new(RefCell::new(rhs_20)));
    root.right = Some(Rc::new(RefCell::new(rhs)));

    println!("{}", max_depth(Some(Rc::new(RefCell::new(root)))));

    let mut root2 = TreeNode::new(1);
    let rhs2 = TreeNode::new(2);
    root2.right = Some(Rc::new(RefCell::new(rhs2)));

    println!("{}", max_depth(Some(Rc::new(RefCell::new(root2)))));

    let mut root3 = TreeNode::new(1);
    let mut rhs3 = TreeNode::new(2);
    let mut rhs3a = TreeNode::new(3);
    let rhs3b = TreeNode::new(4);
    rhs3a.right = Some(Rc::new(RefCell::new(rhs3b)));
    rhs3.right = Some(Rc::new(RefCell::new(rhs3a)));
    root3.right = Some(Rc::new(RefCell::new(rhs3)));

    println!("{}", max_depth(Some(Rc::new(RefCell::new(root3)))));
}
