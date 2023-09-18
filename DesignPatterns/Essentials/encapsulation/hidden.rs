/* ------------------------------ Encapsulation ----------------------------- */

// In software systems, 
// encapsulation refers to the bundling of data with the mechanisms or methods that operate on the data. 
// It may also refer to the limiting of direct access to some of that data, such as an object's components. 
// 
// Encapsulation allows developers to present a consistent and usable interface 
// which is independent of how a system is implemented internally. 
// 
// As one example, encapsulation can be used to hide the values or state of a structured data object 
// inside a class, preventing direct access to them by clients in a way that could expose 
// hidden implementation details or violate state invariance maintained by the methods.


/* ------------------------------------ x ----------------------------------- */

// Not good encapsulation
pub struct OpenBox<T> {
    pub contents: T,
}

// Good encapsulation
pub struct ClosedBox<T> {
    contents: T,
}

impl<T: std::fmt::Debug + Clone> ClosedBox<T> {
    pub fn new(contents: T) -> ClosedBox<T> {
        ClosedBox { contents: contents }
    }

    // getter
    pub fn get_content(&self) -> T{
        self.contents.clone()
    }

    pub fn set_content(&mut self, content: T){
        self.contents = content
    }
}

