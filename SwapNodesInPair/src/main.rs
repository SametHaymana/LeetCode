
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
    pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        // Vector that will store 4 Node that will be swapped 2 and change the next pointer to the next 2 nodes
        let mut nodes: [Option<Box<ListNode>>; 4] = [None, None, None, None];

        let mut tail = &mut head;



        while let Some(mut node) = tail.take() {
            for i in 0..4 {
                nodes[i] = Some(node.clone());
                node = match node.next {
                    Some(n) => n,
                    None => break,
                };
            }

            if nodes[0].is_none() {
                break;
            }

            nodes[0].as_mut().unwrap().next = nodes[3].take();
            nodes[2].as_mut().unwrap().next = nodes[1].take();
            nodes[3].as_mut().unwrap().next = nodes[2].take();


        }

        head  
    }
}


fn main() {
    println!("Hello, world!");
}
