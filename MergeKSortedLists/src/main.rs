use std::collections::BinaryHeap;
use std::cmp::Reverse;



#[derive(PartialEq, Eq, PartialOrd, Ord,Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}


struct Solution;

impl Solution {

    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        
        let mut result_head = Box::new(ListNode::new(0));
        let mut result = &mut result_head;

        let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();

        for list in lists {

            let mut current = list;
            while let Some(node) = current {
                heap.push(Reverse(node.val));
                current = node.next;
            }
        }

        while let Some(Reverse(val)) = heap.pop(){
            result.next = Some(Box::new(ListNode::new(val)));
            result = result.next.as_mut().unwrap();
        }



        result_head.next
    }
}



// Utility function to convert a vector to a linked list
fn vec_to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    for &val in vec.iter().rev() {
        let mut new_node = ListNode::new(val);
        new_node.next = head;
        head = Some(Box::new(new_node));
    }
    head
}

// Utility function to print a linked list
fn print_list(list: &Option<Box<ListNode>>) {
    let mut current = list;
    while let Some(node) = current {
        print!("{} ", node.val);
        current = &node.next;
    }
    println!();
}

fn main() {
    let test_cases = vec![
        vec![vec![1, 4, 5], vec![1, 3, 4], vec![2, 6]], // Multiple lists
        vec![vec![], vec![0]], // One empty list and one non-empty list
        vec![vec![]], // Single empty list
        vec![vec![1], vec![0]], // Two lists with one element each
    ];

    for case in test_cases {
        let lists: Vec<Option<Box<ListNode>>> = case.into_iter().map(vec_to_list).collect();
        let merged_list = Solution::merge_k_lists(lists);
        print_list(&merged_list);
    }
}
