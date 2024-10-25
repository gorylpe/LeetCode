// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

struct Solution;

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut a = l1.clone();
        let mut b = l2.clone();

        let mut head = Box::new(ListNode::new(0));
        let mut curr = &mut head;
        let mut carry = 0;
        while a.is_some() || b.is_some() || carry != 0 {
            let mut sum = carry;
            if let Some(x) = a {
                sum += x.val;
                a = x.next;
            }

            if let Some(x) = b {
                sum += x.val;
                b = x.next;
            }

            let val = sum % 10;
            carry = sum / 10;
            curr.next = Some(Box::new(ListNode::new(val)));
            curr = curr.next.as_mut().unwrap();
        }
        head.next
    }
}

fn main() {
    assert_eq!(list_to_vec(Solution::add_two_numbers(vec_to_list(vec![2, 4, 3]), vec_to_list(vec![5, 6, 4]))), vec![7, 0, 8]);
    assert_eq!(list_to_vec(Solution::add_two_numbers(vec_to_list(vec![0]), vec_to_list(vec![0]))), vec![0]);
    assert_eq!(list_to_vec(Solution::add_two_numbers(vec_to_list(vec![9, 9, 9, 9, 9, 9, 9]), vec_to_list(vec![9, 9, 9, 9]))), vec![8, 9, 9, 9, 0, 0, 0, 1]);
    assert_eq!(list_to_vec(Solution::add_two_numbers(vec_to_list(vec![9, 9, 9, 9]), vec_to_list(vec![9, 9, 9, 9, 9, 9, 9]))), vec![8, 9, 9, 9, 0, 0, 0, 1]);
}

fn vec_to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut prev: Option<Box<ListNode>> = None;
    for i in vec.iter().rev() {
        let next = Some(Box::new(ListNode { val: *i, next: prev.take() }));
        prev = next;
    }
    prev
}

fn list_to_vec(list: Option<Box<ListNode>>) -> Vec<i32> {
    let mut v = vec![];
    let mut curr = list.clone();
    while curr.is_some() {
        v.push(curr.clone().unwrap().val);
        let next = curr.clone().unwrap().next.clone();
        curr = next
    }
    v
}
