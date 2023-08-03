Say we have a library crate with a module parent, which contains a sub-module child:

└─ library root
   └─ parent
      └─ child


The crate root is in a lib.rs file in the src directory. 
However, 
- the parent module can be either in a parent.rs file next to lib.rs, 
- or in a mod.rs file in a parent directory:

File tree A
├─ Cargo.toml
└─ src/
   ├─ lib.rs
   ├─ parent.rs  // parent module
   └─ parent/
      └─ child.rs


File tree B
├─ Cargo.toml
└─ src/
   ├─ lib.rs
   └─ parent/
      ├─ mod.rs  // parent module
      └─ child.rs
