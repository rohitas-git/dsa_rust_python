#![allow(dead_code)]

mod state_pattern;
mod state_pattern2;

fn main() {}





/*================================ ============ ==============================*/

// Composition is more important in Rust for the obvious reason that 
// you can't inherit functionality in a lazy way from a base class.

// You always have a choice: 
// polymorphic, via trait objects, 
// or monomorphic, via generics constrainted by traits.

// Modern C++ and the Rust standard library tends to take the generic route, 
// but the polymorphic route is not obsolete. 
// 
// You do have to understand the different trade-offs 
// - generics generate the fastest code, which can be inlined. 
// This may lead to code bloat. 
// 
// But not everything needs to be as fast as possible 
// - it may only happen a 'few' times in the lifetime of a typical program run.