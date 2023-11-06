use crate::tree::bst::def_bst::*;

// Problem:
// Invert the binary tree i.e output its mirror image

// Ideas:
// - Mutate current bst in place [Better]
// - Output a new bst that is mirror image of given bst

mod solution {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type MaybeNode = Option<Rc<RefCell<TreeNode>>>;
    
    pub fn invert_tree(mut root: MaybeNode) -> MaybeNode {
        fn helper(node: &mut MaybeNode) {
            if let Some(_node) = node {
                let mut _node = _node.borrow_mut();

                match (_node.left.take(), _node.right.take()) {
                    (None, None) => {}
                    (l, r) => {
                        _node.left = r;
                        _node.right = l;
                    }
                }
                helper(&mut _node.left);
                helper(&mut _node.right);
            }
        }
        helper(&mut root);
        root
    }
}
