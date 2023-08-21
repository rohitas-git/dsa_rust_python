# Quick Sort
# 1. Divide and Conquer Algorithm
#       - In merge sort, division is into two equal parts
#       - In quick sort, division is done by parition function
# 2. Worst case time O(N^2)
# 3. Despite O(n^2) worst case, it is considered faster, 
#      Because of the following reasons
#       - In-place
#       - Cache Friendly
#       - Average case is O(N logN)
#       - Tail Recursive
# 4. Parition is a key function in quick sort [Naive< Lomuto < Hoare]
# (whereas in merge sort, merge fn is complex)

# C++ sort, C qsort, Java primitive type use a variation of quick sort (introSort)
# C++ stable sort, java non-primitive type use a variation of mergesort (timSort)

# When you don't need stability, QuickSort is the fastest algorithm
# When you need stability, MergeSort is prefered

# Quick Sort Lomuto Parition's recursion tree structure matches with Binary Search tree structure

