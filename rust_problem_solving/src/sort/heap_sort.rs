/* -------------------------------- Heap Sort ------------------------------- */
// Heap sort is a comparison based sorting technique based on Binary Heap data structure. 
// It is similar to selection sort where we first find the maximum element and place the maximum element at the end. 
// We repeat the same process for remaining elements.

// => Can be seen as an optimization over Selection sort

// Two steps:
//  1. Build a max heap
//  2. Repeatedly swap root with the last node, reduce heap size by 1 and heapify

// Time - O(N*logN)
// Aux Space - O(1) 
// Used in hybrid sorting algorithms like IntroSort

// Heap sort is an in-place algorithm.
// Its typical implementation is not stable, but can be made stable.
