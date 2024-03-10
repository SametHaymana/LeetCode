use std::rc::Rc;
use std::cell::RefCell;

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
      right: None
    }
  }
}

struct BSTIterator{
    current: Option<Rc<RefCell<TreeNode>>> 
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl  BSTIterator {

    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        BSTIterator{ current: root }
    }
    
    fn next(&self) -> i32 {
        
        match  self.current {
            Some(None) =>{
                

            },
            None => return,
        }
    }
    
    fn has_next(&self) -> bool {
        
    }
}


fn main(){

}


/// * Your BSTIterator object will be instantiated and called as such:
///  * let obj = BSTIterator::new(root);
///  * let ret_1: i32 = obj.next();
///  * let ret_2: bool = obj.has_next();