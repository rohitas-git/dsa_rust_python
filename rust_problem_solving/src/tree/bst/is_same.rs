// Problem:
// Given the roots of two binary trees p and q,
// write a function to check if they are the same or not.

// Two binary trees are considered the same
// if they are structurally identical, and the nodes have the same value.

//---------------------------------------------------------------
use crate::tree::bst::def_bst::*;

struct Solution;

mod solution2 {
    use super::*;

    // This looks a lot nicer! We're able to dodge most of the Rc<RefCell> stuff (just one immutable borrow) 
    // because the compiler is able to reason about lifetimes more easily 
    // when we store the references on the call stack like this.
    
    use std::cell::RefCell;
    use std::rc::Rc;

    type MaybeNode = Option<Rc<RefCell<TreeNode>>>;

    impl Solution {
        pub fn is_same_tree_recursive(p: MaybeNode, q: MaybeNode) -> bool {
            fn helper(p: &MaybeNode, q: &MaybeNode) -> bool {
                match (p, q) {
                    (Some(p), Some(q)) if p == q => {
                        let p = p.borrow();
                        let q = q.borrow();

                        helper(&p.left, &q.left) && helper(&p.right, &q.right)
                    }
                    (None, None) => true,
                    _ => false,
                }
            }

            helper(&p, &q)
        }
    }
}

pub mod solution1 {
    use super::*;

    use std::cell::RefCell;
    use std::rc::Rc;

    type MaybeNode = Option<Rc<RefCell<TreeNode>>>;

    impl Solution {
        pub fn is_same_tree_imperative(p: MaybeNode, q: MaybeNode) -> bool {
            let mut stack = vec![];
            stack.push((p, q));
            while !stack.is_empty() {
                // unwrapping is safe because of our while condition
                let pair = stack.pop().unwrap();
                match pair {
                    (Some(p), Some(q)) if p == q => {
                        stack.push((p.borrow().left.clone(), q.borrow().left.clone()));
                        stack.push((p.borrow().right.clone(), q.borrow().right.clone()));
                    }
                    (None, None) => {}
                    _ => {
                        return false;
                    }
                }
            }
            true
        }
    }
}
