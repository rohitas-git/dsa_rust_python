// ERROR: recursive type `first::List` has infinite size
// pub enum List {
//     Empty,
//     Elem(i32, List),
// }

// inserted indirection (e.g., a Box, Rc, or &) at some point to make first::List representable

// layout 1: 
// #[derive(Debug)]
// enum List<T> {
//     Elem(T, Box<List<T>>),
//     Empty,
// }

// List<T> 
//-> is actually a really foolish definition of a List, for a few reasons.
// 1. Junk 
// 2. Non-uniform node layout

// Consider a list with two elements:
//      [] = Stack
//      () = Heap
//      [Elem A, ptr] -> (Elem B, ptr) -> (Empty, *junk*)


// consider the following potential layout for our list:
//      [ptr] -> (Elem A, ptr) -> (Elem B, *null*)

// The key difference is the absence of the junk from our first layout. 

// 1. What is this junk?
// Memory layout of Enum:
// <Tag of enum> : some integer to indicate which variant of the enum 
// <Largest variant memory>: also need enough space to store the largest of the variants
// (plus some extra space to satisfy alignment requirements).
// 
// Junk: Resultant extra waste space, due to used enum variant not being the Largest.
// Enum variant does not occupy less space if a smaller sized variant is used. 
// Therefore the first layout heap allocates an extra element that's just full of junk, consuming a bit more space than the second layout.

// 2. One of our nodes not being allocated at all is also, 
//      perhaps surprisingly, worse than always allocating it. 
// This is because it gives us a non-uniform node layout. 
// This doesn't have much of an appreciable effect on pushing and popping nodes, 
//      but it does have an effect on splitting and merging lists.

// Consider splitting a list in both layouts:

// layout 1:
// [Elem A, ptr] -> (Elem B, ptr) -> (Elem C, ptr) -> (Empty *junk*)
// 
// split off C:
// [Elem A, ptr] -> (Elem B, ptr) -> (Empty *junk*)
// [Elem C, ptr] -> (Empty *junk*)

// layout 2:
// [ptr] -> (Elem A, ptr) -> (Elem B, ptr) -> (Elem C, *null*)
// 
// split off C:
// [ptr] -> (Elem A, ptr) -> (Elem B, *null*)
// [ptr] -> (Elem C, *null*)

// One of the few nice things about a linked list is 
// that you can construct the element in the node itself, 
// and then freely shuffle it around lists without ever moving it. 
// You just fiddle with pointers and stuff gets "moved". 
// Layout 1 trashes this property.

pub enum List {
    Empty,
    ElemThenEmpty(i32),
    ElemThenNotEmpty(i32, Box<List>),
}

// Above layout doesn't support null pointer optimization and waste even more space.
//  There are few more reason that it's worse than layout 2.

// Null pointer optimization
// It means &, &mut, Box, Rc, Arc, Vec, and several other important types in Rust 
// have no overhead when put in an Option! 

