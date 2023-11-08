// So how do we avoid the extra junk, uniformly allocate,
// and get that sweet null-pointer optimization?
// > We need to better separate out the idea of having an element from allocating another list.

// enums let us declare a type that can contain one of several values
// structs let us declare a type that contains many values at once
// > Let's break our List into two types: A List, and a Node.

// a List is either Empty or has an element followed by another List.
// By representing the "has an element followed by another List" case by an entirely separate type,
// we can hoist the Box to be in a more optimal position:

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

// Let's check our priorities:
// Tail of a list never allocates extra junk: check!
// enum is in delicious null-pointer-optimized form: check!
// All elements are uniformly allocated: check!

// Because List is a struct with a single field, 
// its size is the same as that field. Yay zero-cost abstractions!