/**========================================================================
 *           Inheritance as a Type System and as Code Sharing
 *========================================================================**/

// Implementation Inheritance is a mechanism whereby 
// an object can inherit elements from another object’s definition, 
// thus gaining the parent object’s data and behavior without you having to define them again.

// Rust doesn't have Implementation inheritance

// You would choose inheritance for two main reasons. 

// * - One is for reuse of code:
//      you can implement particular behavior for one type, 
//      and inheritance enables you to reuse that implementation for a different type. 
// * You can do this in a limited way in Rust code using default trait method implementations,

// * - The other reason to use inheritance relates to the type system:
//      to enable a child type to be used in the same places as the parent type. 
// 
//      This is also called polymorphism, 
//      which means that you can substitute multiple objects for each other at runtime 
//      if they share certain characteristics.
// * Rust allows polymorphism through generics and trait bounds

/**----------------------
 *    Polymorphism
 *------------------------**/
//  To many people, polymorphism is synonymous with inheritance. 
// But it’s actually a more general concept that refers to code that can work with data of multiple types. 
// For inheritance, those types are generally subclasses.

// Rust instead uses generics to abstract over different possible types and trait bounds to impose constraints on what those types must provide. 
// This is sometimes called bounded parametric polymorphism.

/**========================
 *    Fallen Inheritance
 *=========================**/
// Inheritance has recently fallen out of favor as a programming design solution in many programming languages 
// because it’s often at risk of sharing more code than necessary. 
// Subclasses shouldn’t always share all characteristics of their parent class but will do so with inheritance. 
// This can make a program’s design less flexible.

/**============================================
 *               Monomorphization
 *=============================================**/

// A process which happens when we use generics and trait bound.
// A process where compiler will generate non-generic impls of fns 
// based on the concrete types used in place of generic types
// What happens is that we substitute concrete impls inplace of generic implementations

// monomorphization process performed by the compiler when we use trait bounds on generics:
// the compiler generates nongeneric implementations of functions and methods for each concrete type that we use in place of a generic type parameter. 
// 
// The code that results from monomorphization is doing static dispatch, 
// which is when the compiler knows what method you’re calling at compile time. 
// 
// This is opposed to dynamic dispatch, 
// which is when the compiler can’t tell at compile time which method you’re calling. 
// In dynamic dispatch cases, the compiler emits code that at runtime will figure out which method to call.

/**======================
 *    Static Dispatch
 *========================**/

// Static dispatch is when the compiler knows the concrete functions you are calling
// at compile time

/**======================
 *    Dynamic Dispatch
 *========================**/

// Dynamic dispatch is when the compiler doesn't knows the concrete functions 
// you are calling at compile time and istead figures it out at runtime

// When we use trait objects, Rust must use dynamic dispatch. 
// The compiler doesn’t know all the types that might be used with the code that’s using trait objects, 
// so it doesn’t know which method implemented on which type to call. 
// Instead, at runtime, Rust uses the pointers inside the trait object to know which method to call. 
// This lookup incurs a runtime cost that doesn’t occur with static dispatch. 
// Dynamic dispatch also prevents the compiler from choosing to inline a method’s code, which in turn prevents some optimizations. 
// However, we did get extra flexibility in the code
