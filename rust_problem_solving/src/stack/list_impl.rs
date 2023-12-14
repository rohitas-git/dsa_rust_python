use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::fmt::{Debug, Display};
use std::rc::Rc;

pub struct Node<T> {
    data: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> where T: Default{
    fn new(data: T) -> Self{
        Node{
            data, 
            next: None
        }
    }

    fn default() -> Self{
        Node{
            data: T::default(), 
            next: None
        }
    }
}

pub struct Stack<T>(Option<Rc<RefCell<Node<T>>>>);

#[derive(Debug)]
pub enum StackError {
    OutOfBounds,
}

impl<T> Stack<T>
where
    T: Debug + Display + Default,
{
    fn new() -> Self {
        Stack(None)
    }

    fn push(&mut self, data: T) -> Result<(), StackError> {
        if let Some(node) = self.0.as_ref() {
            let mut node_ref = &node.borrow();
            while node_ref.next.is_some() {
                node_ref = &node_ref.next.unwrap().borrow();
            }
            let new_node = Node::new(data);
            node_ref.borrow_mut().next = Some(Rc::new(RefCell::new(new_node)));
        }

        Ok(())
    }

    fn pop(&mut self) -> Result<(), StackError> {
        if let Some(node) = self.0.as_ref() {
            let mut node_ref = node.borrow();
            while node_ref.next.unwrap().borrow().next.is_some() {
                node_ref = node_ref.next.unwrap().borrow();
            }
            node_ref.borrow_mut().next = None; 
        }

        Err(StackError::OutOfBounds)
    }

    fn top(&self) -> Result<(), StackError> {
        if let Some(node) = self.0.as_ref() {
            let mut node_ref = node.borrow();
            println!("Data at top: {:?}", node_ref.data);
        }
        Err(StackError::OutOfBounds)
    }

    fn len(&self) -> usize {
        if let Some(node) = self.0.clone() {
            let mut node_ref = node.clone();
            let mut count = 0;
            while (node_ref.borrow().next).is_some() {
                let x = node_ref;
                // node_ref = node_ref.next.as_ref().unwrap();
                count+=1;
            }
            return count; 
        }
        0
    }

    fn display(&self) {
        println!("{}", self);
    }
}

impl<T: Display> Display for Stack<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for item in self.0.iter() {
            write!(f, "{}->", item.borrow().data)?;
        }
        Ok(())
    }
}
