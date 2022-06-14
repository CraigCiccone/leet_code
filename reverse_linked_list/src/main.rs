// EASY
//
// Given the head of a singly linked list, reverse the list, and return the reversed list.
//
// Example 1:
//
// Input: head = [1,2,3,4,5]
// Output: [5,4,3,2,1]
//
// Example 2:
//
// Input: head = [1,2]
// Output: [2,1]
// Example 3:
//
// Input: head = []
// Output: []
//
// Constraints:
//
// The number of nodes in the list is the range [0, 5000].
// -5000 <= Node.val <= 5000

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut reverse_head = None;
    let mut iter = head;

    while let Some(mut node) = iter {
        iter = node.next;
        node.next = reverse_head;
        reverse_head = Some(node);
    }

    reverse_head
}

fn main() {
    let mut ex1_head = ListNode::new(1);
    let mut ex1_node1 = ListNode::new(2);
    let mut ex1_node2 = ListNode::new(3);
    let mut ex1_node3 = ListNode::new(4);
    let ex1_node4 = ListNode::new(5);
    ex1_node3.next = Some(Box::new(ex1_node4));
    ex1_node2.next = Some(Box::new(ex1_node3));
    ex1_node1.next = Some(Box::new(ex1_node2));
    ex1_head.next = Some(Box::new(ex1_node1));
    println!("{:?}", reverse_list(Some(Box::new(ex1_head))));
    let mut ex2_head = ListNode::new(1);
    let ex2_node1 = ListNode::new(2);
    ex2_head.next = Some(Box::new(ex2_node1));
    println!("{:?}", reverse_list(Some(Box::new(ex2_head))));
    println!("{:?}", reverse_list(None));
}
