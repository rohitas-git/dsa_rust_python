pub trait Draw{
    fn draw(&self);
}

/**============================================
 *               Trait objects
 *=============================================**/

// List of components, Vec<Box<dyn Draw>, 
// can store elements of multiple types given that they impl Draw
// trait objects allow for multiple concrete types to fill in for the trait object at runtime

// We can also created a workaround where we define a DrawObject enum that had variants to hold different struct types.
// However, sometimes we want our library user to be able to extend the set of types that are valid in a particular situation.
// Also, at the time of writing the library, we can’t know and define all the types other programmers might want to create.

// The advantage of using trait objects and Rust’s type system to write code similar to code using duck typing is that 
// we never have to check whether a value implements a particular method at runtime or worry about getting errors if a value doesn’t implement a method but we call it anyway.

/**======================
 *    Dynamic Dispatch
 *========================**/
// * When using trait objects, rust compiler must use dynamic dispatch 
// and that's because compiler doesn't know all the concrete objects that are gonna be used at compile time
// Instead the compiler will add code to figure out the correct method to call at runtime
// 
// Downside: which means that there is a runtime performance cost
// Upside: You get to write flexible code that can accept any object that impls a certain trait

/**======================
 *    Object Safety
 *========================**/
// Only object safe traits can be made into trait bounds

// What does it mean?
// A trait is object safe when all the methods implementation on that trait have these two properties:
// 1. The return type in not self
// 2. There are no generic parameters

// If a trait doesn't haves these two properties then rust compiler can't figure out 
// the concrete type of that trait and therefore, doesn't know the correct method to call

mod trait_objects{
    use super::Draw;

    pub struct Screen{
        pub components: Vec<Box<dyn Draw>
    }

    impl Screen{
        pub fn run(&self){
            for component in self.components.iter(){
                component.draw();
            }
        }
    }
}

/**============================================
 *               Generics
 *=============================================**/

// List of components, Vec<T>,
// can only store homogeneous (of one type) elements given its type impl Draw
// A generic type parameter can only be substituted with one concrete type at a time

// If you’ll only ever have homogeneous collections, using generics and trait bounds is preferable 
// because the definitions will be monomorphized at compile time to use the concrete types.

/**======================
 *    Static Dispatch
 *========================**/
// Static dispatch is when the compiler knows the concrete functions you are calling
// at compile time

mod generics{
    use super::Draw;

    pub struct Screen<T: Draw>{
        pub components: Vec<T>
    }

    impl<T> Screen<T> where T:Draw{
        pub fn run(&self){
            for component in self.components.iter(){
                component.draw();
            }
        }
    }
}

/**===================================================================================
 *                      If used Implementation Inheritance
 * 
 * To do this in a language with inheritance, 
 * we might define a class named Component that has a method named draw on it. 
 * The other classes, such as Button, Image, and SelectBox, would inherit from Component and thus inherit the draw method. 
 * They could each override the draw method to define their custom behavior, 
 * but the framework could treat all of the types as if they were Component instances and call draw on them. 
 * 
 * But because Rust doesn’t have inheritance, 
 * we need another way to structure the gui library to allow users to extend it with new types.
 *  
 *=====================================================================================**/