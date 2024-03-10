use std::borrow::Borrow;


#[derive(PartialEq, Eq, Clone, Debug)]
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
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
      let mut node1 = list1;
      let mut node2 = list2;


      let mut result_head:Box<ListNode> = Box::new(ListNode::new(0));
      let mut result_tail= &mut result_head;



      while node1.is_some() || node2.is_some(){

        match (node1.take(), node2.take()) {
          (Some(n1), Some(n2)) => {
            if n1.val < n2.val {
              result_tail.next = Some(n1.clone());
              result_tail = result_tail.next.as_mut().unwrap();
              node1 = n1.next;
              node2 = Some(n2);
            } else {
              result_tail.next = Some(n2.clone());
              result_tail = result_tail.next.as_mut().unwrap();
              node2 = n2.next;
              node1 = Some(n1);
            }
          },
          (Some(n1), None) => {
            result_tail.next = Some(n1.clone());
            result_tail = result_tail.next.as_mut().unwrap();
            node1 = n1.next;
          },
          (None, Some(n2)) => {
            result_tail.next = Some(n2.clone());
            result_tail = result_tail.next.as_mut().unwrap();
            node2 = n2.next;
          },
          (None, None) =>{
            break;
          }
            
        }

      }

      result_head.next

    }
}

fn to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
  let mut current = None;
  for &value in vec.iter().rev() {
      let mut node = ListNode::new(value);
      node.next = current;
      current = Some(Box::new(node));
  }
  current
}

fn print_list(list: Option<Box<ListNode>>) {
  let mut current = &list;
  while let Some(ref node) = *current {
      print!("{} ", node.val);
      current = &node.next;
  }
  println!();
}

fn main() {
  let test_cases = vec![
      (vec![1, 2, 4], vec![1, 3, 4]),
      (vec![], vec![0, 1, 2]),
      (vec![1, 2, 3], vec![]),
      (vec![], vec![]),
      (vec![5, 6, 7], vec![1, 2, 3]),
  ];

  for (vec1, vec2) in test_cases {
      let list1 = to_list(vec1);
      let list2 = to_list(vec2);
      println!("List 1: ");
      print_list(list1.clone());
      println!("List 2: ");
      print_list(list2.clone());
      let merged_list = Solution::merge_two_lists(list1, list2);
      println!("Merged List: ");
      print_list(merged_list);
      println!("-------------------");
  }
}
