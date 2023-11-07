use std::fmt::{Debug, Display};

const STACKMAX: usize = 100;

pub struct Stack<T>(Vec<T>);

#[derive(Debug)]
pub enum StackError {
    Overflow(String),
    Underflow(String),
}

impl<T> Stack<T>
where
    T: Debug + Display,
{
    fn new() -> Self {
        Stack(Vec::<T>::with_capacity(STACKMAX))
    }

    fn push(&mut self, data: T) -> Result<(), StackError> {
        if self.len() < STACKMAX {
            self.0.push(data);
            Ok(())
        } else {
            Err(StackError::Overflow(
                "Overflow: Need to empty stack before push".to_string(),
            ))
        }
    }

    fn pop(&mut self) -> Result<(), StackError> {
        if self.len() > 0 {
            let p = self.0.pop().unwrap();
            println!("Item popped from stack: {:?}", p);
            Ok(())
        } else {
            Err(StackError::Underflow(
                "Underflow: Need to add items to stack before calling pop".to_string(),
            ))
        }
    }

    fn top(&self) -> Result<(), StackError> {
        if self.len() > 0 {
            println!("Top of stack: {:?}", self.0.last());
            Ok(())
        } else {
            Err(StackError::Underflow(
                "Underflow: Need to add items to stack before calling top".to_string(),
            ))
        }
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn display(&self)  {
        println!("{}", self);
    }
}

impl<T: Display> Display for Stack<T>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for item in self.0.iter(){
            write!(f, "{}->", item)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test_stack {
    use super::*;

    #[test]
    fn test_stack_creation() -> Result<(), StackError> {
        let mut my_stack = Stack::<u32>::new();
        my_stack.push(10)?;
        my_stack.push(20)?;
        my_stack.push(30)?;
        my_stack.pop()?;
        my_stack.push(40)?;
        my_stack.push(50)?;

        my_stack.display();
        
        Ok(())
    }
}
