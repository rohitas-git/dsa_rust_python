
[Time, Space] 

O(n^2)/O(n) , O(1):
    Insertion > Bubble > Selection ()

O(nlogN), O(n):
    QuickSort> MergeSort 

O(nlogN), O(1):
    HeapSort
 
Quick sort > Heap sort

C++ sort, C qsort, Java primitive type use a variation of quick sort (IntroSort)
C++ stable sort, java non-primitive type use a variation of mergesort (timSort)

When you don't need stability, QuickSort is the fastest algorithm
When you need stability, MergeSort is prefered

Key fn:
- of MergeSort, Merge fn
- of QuickSort, Parition fn

IntroSort - Insertion sort, Heap sort, Quick sort

## Merge Sort

- Divide and Conquer Algorithm
- Stable Algorithm
- O(N logN) time and O(N) aux space
- Well suited for linked list. Works in O(1) aux space
- Used in external sorting
- In general for arrays, quickSort outperforms it

Merge Sort is a recursive algorithm and time complexity can be expressed as following recurrence relation.
    T(n) = 2T(n/2) + θ(n)

(worst, average, and best) Time - θ(Nlog(N)) from O( logN times N )
as merge sort always divides the array into two halves and takes linear time to merge two halves.

Auxiliary Space: O(n), In merge sort all elements are copied into an auxiliary array. 
So N auxiliary space is required for merge sort.

## Quick Sort

1. Divide and Conquer Algorithm
      - In merge sort, division is into two equal parts
      - In quick sort, division is done by parition function
2. Worst case time O(N^2)
3. Despite O(n^2) worst case, it is considered faster, 
    Because of the following reasons:
        - In-place
        - Cache Friendly
        - Average case is O(N logN)
        - Tail Recursive
4. Parition is a key function
   
Parition fn - Naive < Lomuto < Hoare
Stable - Naive (does 3 traversal of array)
Unstable - Lomuto, Hoare (only do 1 traversal of array)