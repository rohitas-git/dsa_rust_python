So, here's a summary:

- the role played by class is shared between data and traits
- structs and enums are dumb, although you can define methods and do data hiding
- a limited form of subtyping is possible on data using the Deref trait
- traits don't have any data, but can be implemented for any type (not just structs)
- traits can inherit from other traits
- traits can have provided methods, allowing interface code re-use
- traits give you both virtual methods (polymorphism) and generic constraints (monomorphism)
