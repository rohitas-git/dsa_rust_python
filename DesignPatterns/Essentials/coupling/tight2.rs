use std::fmt::Debug;

#[path="./tight.rs"]
mod tight;
use tight::*;

pub struct OpenBox<T> {
    pub contents: T,
}

// OpenBox<T> is tightly coupled with ClosedBox<T>
impl<T: Debug + Clone> OpenBox<T> {

    fn tight_couple(&self, noun: &str) {
        let my_box = ClosedBox::new(noun);
        let content = my_box.get_content();
        ClosedBox::<T>::say_hello(content);
    }
}

fn main() {
    let open_box = OpenBox { contents: "public information" };   
    open_box.tight_couple("Rohitas");
}