// EASY
//
// Given the roots of two binary trees p and q, write a function to check if they are the same or not.
// Two binary trees are considered the same if they are structurally identical, and the nodes have the same value.
//
// Example 1:
//
// Input: p = [1,2,3], q = [1,2,3]
// Output: true
//
// Example 2:
//
// Input: p = [1,2], q = [1,null,2]
// Output: false
//
// Example 3:
//
// Input: p = [1,2,1], q = [1,1,2]
// Output: false
//
// Constraints:
//
// The number of nodes in both trees is in the range [0, 100].
// -10^4 <= Node.val <= 10^4

use std::cell::RefCell;
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

fn traverse(p: &Option<Rc<RefCell<TreeNode>>>, q: &Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (p, q) {
        (None, None) => true,
        (None, Some(_)) => false,
        (Some(_), None) => false,
        (Some(tree1), Some(tree2)) => {
            if tree1.borrow().val != tree2.borrow().val {
                return false;
            }

            if !traverse(&tree1.borrow().left, &tree2.borrow().left)
                || !traverse(&tree1.borrow().right, &tree2.borrow().right)
            {
                return false;
            }

            true
        }
    }
}

fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    traverse(&p, &q)
}

fn main() {
    let mut p1_root = TreeNode::new(1);
    let p1_lhs = TreeNode::new(2);
    let p1_rhs = TreeNode::new(3);
    p1_root.left = Some(Rc::new(RefCell::new(p1_lhs)));
    p1_root.right = Some(Rc::new(RefCell::new(p1_rhs)));
    let mut q1_root = TreeNode::new(1);
    let q1_lhs = TreeNode::new(2);
    let q1_rhs = TreeNode::new(3);
    q1_root.left = Some(Rc::new(RefCell::new(q1_lhs)));
    q1_root.right = Some(Rc::new(RefCell::new(q1_rhs)));
    println!(
        "{}",
        is_same_tree(
            Some(Rc::new(RefCell::new(p1_root))),
            Some(Rc::new(RefCell::new(q1_root)))
        )
    );

    let mut p2_root = TreeNode::new(1);
    let p2_lhs = TreeNode::new(2);
    p2_root.left = Some(Rc::new(RefCell::new(p2_lhs)));
    let mut q2_root = TreeNode::new(1);
    let q2_rhs = TreeNode::new(2);
    q2_root.right = Some(Rc::new(RefCell::new(q2_rhs)));
    println!(
        "{}",
        is_same_tree(
            Some(Rc::new(RefCell::new(p2_root))),
            Some(Rc::new(RefCell::new(q2_root)))
        )
    );

    let mut p3_root = TreeNode::new(1);
    let p3_lhs = TreeNode::new(2);
    let p3_rhs = TreeNode::new(1);
    p3_root.left = Some(Rc::new(RefCell::new(p3_lhs)));
    p3_root.right = Some(Rc::new(RefCell::new(p3_rhs)));
    let mut q3_root = TreeNode::new(1);
    let q3_lhs = TreeNode::new(1);
    let q3_rhs = TreeNode::new(2);
    q3_root.left = Some(Rc::new(RefCell::new(q3_lhs)));
    q3_root.right = Some(Rc::new(RefCell::new(q3_rhs)));
    println!(
        "{}",
        is_same_tree(
            Some(Rc::new(RefCell::new(p3_root))),
            Some(Rc::new(RefCell::new(q3_root)))
        )
    );
}
