

// Lifetime Elision in case of:

trait ElidedLifetimes{
    // Only one reference in input, so the output must be derived from that input
    fn foo(&A) -> &B; // sugar for:
    fn foo<'a>(&'a A) -> &'a B;

    // Many inputs, assume they're all independent
    fn foo(&A, &B, &C); // sugar for:
    fn foo<'a, 'b, 'c>(&'a A, &'b B, &'c C);

    // Methods, assume all output lifetimes are derived from `self`
    fn foo(&self, &B, &C) -> &D; // sugar for:
    fn foo<'a, 'b, 'c>(&'a self, &'b B, &'c C) -> &'a D;

}

