use std::fmt::{Debug, Display};

/* -------------------------------- Approach -------------------------------- */

// The intuition is to fill the array in a circular manner,
// (ie) after popping from the front, rather than moving all the elements towards the front.
// We can have 2 variables to keep track of the start and end indexes of the sequence.
// Mod addition is done to handle boundary conditions.

const QUEUEMAX: usize = 100;

pub struct Queue<T> {
    queue: Vec<T>,
    front: Option<usize>,
    rear: Option<usize>,
}

#[derive(Debug)]
pub enum QueueError {
    Overflow(String),
    Underflow(String),
}

impl<T> Queue<T>
where
    T: Debug + Display,
{
    fn new() -> Self {
        Queue {
            queue: Vec::<T>::with_capacity(QUEUEMAX),
            front: None,
            rear: None,
        }
    }

    fn push_rear(&mut self, data: T) -> Result<(), QueueError> {
        if self.len() < QUEUEMAX {
            self.queue.push(data);

            if let Some(rear) = self.rear {
                self.rear = Some(rear + 1);
            } else {
                self.rear = Some(0)
            }

            if self.front.is_none() {
                self.front = Some(0);
            }

            Ok(())
        } else {
            Err(QueueError::Overflow(
                "Overflow: Need to empty queue before push".to_string(),
            ))
        }
    }

    fn pop_front(&mut self) -> Result<(), QueueError> {
        if self.len() > 0 {
            let p = self.queue.get(self.front.unwrap()).unwrap();

            if let Some(front) = self.front {
                self.front = Some(front + 1);
            }

            println!("Item popped from queue: {:?}", p);
            Ok(())
        } else {
            Err(QueueError::Underflow(
                "Underflow: Need to add items to queue before calling pop".to_string(),
            ))
        }
    }

    fn top(&self) -> Result<(), QueueError> {
        if self.len() > 0 {
            println!(
                "Top of queue: {:?}",
                self.queue.get(self.front.unwrap()).unwrap()
            );
            Ok(())
        } else {
            Err(QueueError::Underflow(
                "Underflow: Need to add items to queue before calling top".to_string(),
            ))
        }
    }

    fn len(&self) -> usize {
        if self.front.is_some() && self.rear.is_some() {
            self.rear.unwrap() - self.front.unwrap() + 1
        } else {
            0
        }
    }

    fn display(&self) {
        println!("{}", self);
    }
}

impl<T: Display + Debug> Display for Queue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.len() > 0 {
            let front = self.front.unwrap();
            let rear = self.rear.unwrap();
            for item in self.queue[front..=rear].iter() {
                write!(f, "{} <- ", item)?;
            }
            Ok(())
        }
        else{
            Err(std::fmt::Error)
        }
    }
}

#[cfg(test)]
mod test_queue {
    use super::*;

    #[test]
    fn test_queue_creation() -> Result<(), QueueError> {
        let mut my_queue = Queue::<u32>::new();
        my_queue.push_rear(10)?;
        my_queue.push_rear(20)?;
        my_queue.push_rear(30)?;
        my_queue.pop_front()?;
        my_queue.push_rear(40)?;
        my_queue.push_rear(50)?;
        my_queue.pop_front()?;

        my_queue.display();

        Ok(())
    }
}
