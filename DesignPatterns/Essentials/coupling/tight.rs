/* ----------------------------- Tight Coupling ----------------------------- */

// One component has dependency on one or more components 
// => Change in one component will result in cascading changes to the dependent component

// Tight coupling is because there are no traits (interfaces) used to connect components

/* ------------------------------------ x ----------------------------------- */
use std::fmt::Debug;

// A public struct with a private field of generic type `T`
pub struct ClosedBox<T> {
    contents: T,
}

impl<T: Debug + Clone> ClosedBox<T> {
    // A public constructor method
    pub fn new(contents: T) -> ClosedBox<T> {
        ClosedBox {
            contents: contents,
        }
    }

    // method
    pub fn get_content(&self) -> T{
        self.contents.clone()
    }

    // fn
    pub fn say_hello(noun: &str){
        println!("Hello {}!", noun);
    }
}

