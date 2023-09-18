/**========================================================================
 *                           Trait Object
 *========================================================================**/
// A trait object points to both 
// - an instance of a type implementing our specified trait and 
// - a table used to look up trait methods on that type at runtime.

// We create a trait object by 
// specifying some sort of pointer, such as  
// - a & reference [ &dyn Draw]
// - a Box<T> smart pointer [Box<dyn Draw>] 
// then the dyn keyword, and then specifying the relevant trait.

// We can use trait objects in place of a generic or concrete type. 
// Wherever we use a trait object, Rust’s type system will ensure at compile time 
// that any value used in that context will implement the trait object’s trait. 
// Consequently, we don’t need to know all the possible types at compile time.

// trait objects are more like objects in other languages in the sense that they combine data and behavior. 
// But trait objects differ from traditional objects in that we can’t add data to a trait object.
// : their specific purpose is to allow abstraction across common behavior.

// Trait Objects ==> Use Dynamic Dispatch

trait Show {
    fn show(&self) -> String;
}

impl Show for i32 {
    fn show(&self) -> String {
        format!("four-byte signed {}", self)
    }
}

impl Show for f64 {
    fn show(&self) -> String {
        format!("eight-byte float {}", self)
    }
}

pub fn use_trait_obj(){
    let answer = 42;
    let maybe_pi = 3.14;
    let v: Vec<&dyn Show> = vec![&answer,&maybe_pi];
    for d in v.iter() {
        println!("show {}",d.show());
    }
}

fn use_trait_obj_boxed(){
    let answer = Box::new(42);
    let maybe_pi = Box::new(3.14);

    let show_list: Vec<Box<dyn Show>> = vec![maybe_pi,answer];
    for d in &show_list {
        println!("show {}",d.show());
    }
}