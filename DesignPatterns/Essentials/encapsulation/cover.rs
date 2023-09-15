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

impl<T: std::fmt::Debug> ClosedBox<T> {
    pub fn new(contents: T) -> ClosedBox<T> {
        ClosedBox { contents: contents }
    }

    pub fn get_content(&self) -> T{
        self.contents.clone()
    }

    pub fn print_content(&self) {
        println!("The close box contains: {:?}", self.contents);
    }
}

fn main() {
    let open_box = OpenBox {
        contents: "public information",
    };
    println!("The open box contains: {}", open_box.contents);

    let _closed_box = ClosedBox::new("classified information");
    ClosedBox::print_content(&_closed_box);
    _closed_box.print_content();
}
