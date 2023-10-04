

## Chaining:
1. Chaining is Simpler to implement.
2. In chaining, Hash table never fills up, we can always add more elements to chain.
3. Chaining is Less sensitive to the hash function or load factors.
4. Chaining is mostly used when it is unknown how many and how frequently keys may be inserted or deleted.
5. Cache performance of chaining is not good as keys are stored using linked list.
6. Wastage of Space (Some Parts of hash table in chaining are never used).
7. Chaining uses extra space for links.
8. Free of deletion problem that is present in open addressing

## Open Addressing:
1. Open Addressing requires more computation.
2. In open addressing, table may become full.
3. Open addressing requires extra care to avoid clustering and load factor.
4. Open addressing is used when the frequency and number of keys is known.
5. Open addressing provides better cache performance as everything is stored in the same table.
6. In Open addressing, a slot can be used even if an input doesn’t map to it.
7. No links in Open addressing
8. Can have problem in deletion of element if empty bucket comes in btw while searching for that element 