
# Disadvantage of Rc<RefCell<Node>> definition
Does this way of modeling trees match what you might see in a real crate? 

It depends. The problem with Rc<RefCell<TreeNode>> is that it can panic at runtime.

For small, controlled problems like what we go through here, you can usually "prove" (by thinking really hard) that your code doesn't panic because the surface area is small. 

For crates that create and work with big, messy trees (like parsers, for example), it's better to stay in the land of compile-time safety. 

I might reach for something like indextree, which does arena allocation using a single Vec as a backing store and uses indices to keep track of all the edges -- skipping RefCell altogether!

# Advantage of Rc<RefCell<Node>> definition
The big (and probably only) advantage of using Rc<RefCell> is that tree nodes can outlive the graph or tree that they come from. This can come in handy for leetcode problems where you want to quickly throw nodes into some other data structure to implement your algorithm. 

A more measured approach that might appear in real software would allocate the data structure required and ensure the lifetimes match up with the arena-allocated tree nodes so that we could freely store their references without the need for reference counting pointers.

# With Box<Node>
It's worth pointing out, too, that for simple use cases you can model trees with Box<T>. Boxes are more straightforward to work with but don't allow shared ownership, and won't work for general graphs that might have cycles (which can also cause problems for Rc!).

# indextree crate
https://crates.io/crates/indextree