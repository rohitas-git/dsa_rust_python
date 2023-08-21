from mergeSubArrays import mergeSubarrays

# 1. Recursively sort Left half and Right Half
# 2. Merge left and right half in a sorted manner

# Merge Sort
# - Divide and Conquer Algorithm
# - Stable Algorithm
# - O(N logN) time and O(N) aux space
# - Well suited for linked list. Works in O(1) aux space
# - Used in external sorting
# - In general for arrays, quickSort outperforms it

# Merge Sort is a recursive algorithm and time complexity can be expressed as following recurrence relation.
# T(n) = 2T(n/2) + θ(n)

# The time complexity of Merge Sort isθ(Nlog(N)) in all 3 cases (worst, average, and best) 
# as merge sort always divides the array into two halves and takes linear time to merge two halves.

# Auxiliary Space: O(n), In merge sort all elements are copied into an auxiliary array. 
# So N auxiliary space is required for merge sort.

def mergeSort(arr,l,r):
    if r > l:
        m = (r+l)/2
        
        mergeSort(arr,l,m)
        mergeSort(arr,m+1,r)
        
        mergeSubarrays(arr,l,m,r)
        