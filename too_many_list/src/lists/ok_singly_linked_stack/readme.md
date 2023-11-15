

This is marginally better, but the big wins will come from Option's methods.

First, mem::replace(&mut option, None) is such an incredibly common idiom that Option actually just went ahead and made it a method: take.


https://rust-unofficial.github.io/too-many-lists/second-into-iter.html

pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}


Here's actually 3 different kinds of iterator each collection should endeavour to implement:

IntoIter - T
IterMut - &mut T
Iter - &T


opy types are known to be perfectly copyable by a bitwise copy. As such, they have a super power: when moved, the old value is still usable. As a consequence, you can even move a Copy type out of a reference without replacement!

