// What's a linked list?
// - Well basically, it's a bunch of pieces of data on the heap (hush, kernel people!)
//      that point to each other in sequence
// - Linked lists are something procedural programmers shouldn't touch with a 10-foot pole, and what functional programmers use for everything.
// - ask functional programmers for the definition of a linked list
//      => List a = Empty | Elem a (List a)

// In Rust, it's equivalent =>
// pub enum List {
//     Empty,
//     Elem(i32, List),
// }
// ERROR: recursive type 'List' has infinite size
//  ---- recursive without indirection
// help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to break the cycle

// Explaination:
// When defining a recursive struct or enum, any use of the type being defined
// from inside the definition must occur behind a pointer (like `Box`, `&` or`Rc`).
// This is because structs and enums must have a well-defined size, and
// without the pointer, the size of the type would need to be unbounded.

// BOX: let x = Box::new(5);
// Box<T>, casually referred to as a 'box', provides the simplest form of heap allocation in Rust. 
// Boxes provide ownership for this allocation, and drop their contents when they go out of scope.

#[derive(Debug)]
pub enum List {
    Empty,
    Elem(i32, Option<Box<List>>),
}

// it built!
// ...but this is actually a really foolish definition of a List, for a few reasons.
// Consider a list with two elements:
//      [] = Stack
//      () = Heap
//      [Elem A, ptr] -> (Elem B, ptr) -> (Empty, *junk*)
// There are two key issues:
//      We're allocating a node that just says "I'm not actually a Node"
//      One of our nodes isn't heap-allocated at all.
// The following potential layout for our list:
//      [ptr] -> (Elem A, ptr) -> (Elem B, *null*)
// 


