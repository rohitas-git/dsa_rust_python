https://rust-unofficial.github.io/too-many-lists/index.html

// A Bad Singly-Linked Stack
// An Ok Singly-Linked Stack
// A Persistent Singly-Linked Stack
// A Bad But Safe Doubly-Linked Deque
// An Unsafe Singly-Linked Queue
// .TODO: An Ok Unsafe Doubly-Linked Deque
// Bonus: A Bunch of Silly Lists

Now of course there's several great use cases for a linked list:
- You want to do a lot of splitting or merging of big lists. A lot.
- You're doing some awesome lock-free concurrent thing.
- You're writing a kernel/embedded thing and want to use an intrusive list.
- You're using a pure functional language and the limited semantics and absence of mutation makes linked lists easier to work with.
- ... and more!

But all of these cases are super rare for anyone writing a Rust program. 99% of the time you should just use a Vec (array stack), and 99% of the other 1% of the time you should be using a VecDeque (array deque). These are blatantly superior data structures for most workloads due to less frequent allocation, lower memory overhead, true random access, and cache locality.

We should all as a community say no to linked lists as a "standard" data structure. It's a fine data structure with several great use cases, but those use cases are exceptional, not common.

Unless you have a workload that is heavily dominated by splitting and merging costs, the penalty every other operation takes due to caching effects and code complexity will eliminate any theoretical gains.