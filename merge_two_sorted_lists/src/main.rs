// You are given the heads of two sorted linked lists list1 and list2.
// Merge the two lists in a one sorted list. The list should be made by splicing together the nodes of the first two lists.
// Return the head of the merged linked list.
//
// Example 1:
// Input: list1 = [1,2,4], list2 = [1,3,4]
// Output: [1,1,2,3,4,4]
// Example 2:
//
// Input: list1 = [], list2 = []
// Output: []
//
// Example 3:
//
// Input: list1 = [], list2 = [0]
// Output: [0]
//
// Constraints:
//
// The number of nodes in both lists is in the range [0, 50].
// -100 <= Node.val <= 100
// Both list1 and list2 are sorted in non-decreasing order.

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

fn merge(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
    next: &mut Option<Box<ListNode>>,
) -> &mut Option<Box<ListNode>> {
    if list1.is_some() && list2.is_none() {
        next.replace(Box::new(ListNode::new(list1.as_ref().unwrap().val)));
        merge(list1.unwrap().next, list2, &mut next.as_mut().unwrap().next);
    } else if list1.is_none() && list2.is_some() {
        next.replace(Box::new(ListNode::new(list2.as_ref().unwrap().val)));
        merge(list1, list2.unwrap().next, &mut next.as_mut().unwrap().next);
    } else if list1.is_some() && list2.is_some() {
        if list1.as_ref().unwrap().val <= list2.as_ref().unwrap().val {
            next.replace(Box::new(ListNode::new(list1.as_ref().unwrap().val)));
            merge(list1.unwrap().next, list2, &mut next.as_mut().unwrap().next);
        } else {
            next.replace(Box::new(ListNode::new(list2.as_ref().unwrap().val)));
            merge(list1, list2.unwrap().next, &mut next.as_mut().unwrap().next);
        }
    }

    next
}

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    merge(list1, list2, &mut None).take()
}

fn main() {
    // Example 1
    let mut e1_l1_a = ListNode::new(1);
    let mut e1_l1_b = ListNode::new(2);
    let e1_l1_c = ListNode::new(4);
    e1_l1_b.next = Some(Box::new(e1_l1_c));
    e1_l1_a.next = Some(Box::new(e1_l1_b));
    let mut e1_l2_a = ListNode::new(1);
    let mut e1_l2_b = ListNode::new(3);
    let e1_l2_c = ListNode::new(4);
    e1_l2_b.next = Some(Box::new(e1_l2_c));
    e1_l2_a.next = Some(Box::new(e1_l2_b));
    println!(
        "{:?}",
        merge_two_lists(Some(Box::new(e1_l1_a)), Some(Box::new(e1_l2_a)))
    );

    // Example 2
    println!("{:?}", merge_two_lists(None, None));

    // Example 3
    let e3_l2_a = ListNode::new(0);
    println!("{:?}", merge_two_lists(None, Some(Box::new(e3_l2_a))));
}
