extern crate rand;

use rand::Rng;

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

pub struct Solution {}

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn carried(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>, mut carry: i32) -> Option<Box<ListNode>> {
            if l1.is_none() && l2.is_none() && carry == 0 {
                None
            } else {
                Some(Box::new(ListNode {
                    next: carried(
                        l1.and_then(|x| {
                            carry += x.val;
                            x.next
                        }),
                        l2.and_then(|x| {
                            carry += x.val;
                            x.next
                        }),
                        carry / 10,
                    ),
                    val: carry % 10,
                }))
            }
        }
        carried(l1, l2, 0)
    }
}

fn main() {
    println!("Hello, world!");
    let mut rng = rand::thread_rng();
    let l1 = Some(Box::new(ListNode { next: None, val: rng.gen::<i32>() }));
    let l2 = Some(Box::new(ListNode { next: None, val: rng.gen::<i32>() }));
    // for i in 0..100000 {}
    println!("{:?}", Solution::add_two_numbers(l1, l2));
}
