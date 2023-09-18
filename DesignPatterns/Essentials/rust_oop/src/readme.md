# OOP in Rust 
https://stevedonovan.github.io/rust-gentle-intro/object-orientation.html 

## General OOP
Everyone comes from somewhere, and the chances are good that your previous programming language implemented Object-Oriented Programming (OOP) in a particular way:

'classes' act as factories for generating objects (often called instances) and define unique types.
Classes may inherit from other classes (their parents), inheriting both data (fields) and behaviour (methods)
If B inherits from A, then an instance of B can be passed to something expecting A (subtyping)
An object should hide its data (encapsulation), which can only be operated on with methods.
Object-oriented design is then about identifying the classes (the 'nouns') and the methods (the 'verbs') and then establishing relationships between them, is-a and has-a.

## Rust flavored OOP 

Rust data aggregates (structs, enums and tuples) are dumb. You can define methods on them, and make the data itself private, all the usual tactics of encapsulation, but they are all unrelated types. There is no subtyping and no inheritance of data (apart from the specialized case of Deref coercions.)

The relationships between various data types in Rust are established using traits. A large part of learning Rust is understanding how the standard library traits operate, because that's the web of meaning that glues all the data types together.

Traits are interesting because there's no one-to-one correspondence between them and concepts from mainstream languages. It depends if you're thinking dynamically or statically. In the dynamic case, they're rather like Java or Go interfaces.

> So Rust traits allow traditional polymorphic OOP

But what about inheritance? People usually mean implementation inheritance whereas Rust does interface inheritance. It's as if a Java programmer never used extend and instead used implements. And this is actually recommended practice by Alan Holub.

Getting the distinction between implementation and interface inheritance is important when understanding Rust.

Implementation inheritance is a relationship where a child class inherits behaviour implementation from a base class.

Interface inheritance is when a child class only inherits the description of behaviour from the base class and provides the implementation itself.