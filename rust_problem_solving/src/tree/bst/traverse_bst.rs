
use crate::tree::bst::def_bst::*; 
use std::cell::RefCell;
use std::rc::Rc;

type MaybeNode = Option<Rc<RefCell<TreeNode>>>;

// Recursive
fn traverse(root: &MaybeNode) {
    if let Some(node) = root {

        let node = node.borrow();

        // PREORDER -- do something here

        traverse(&node.left);

        // INORDER -- do something here

        traverse(&node.right);

        // POSTORDER -- do something here

        // optionally, return a value or something
    }
}

// Iterative approaches should also work fine at the (minimal) cost of extra cloning.

// A more idiomatic approach would probably create an IterMut object 
// that lets us iterate over the tree nodes mutably (and can hide all the reference counting stuff).