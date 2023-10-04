def selectionSort(l):
    n = len(l)
    
    for i in range(n-1):
        min_id = i
        # find the minimum element in unsorted sub-array
        for j in range(i,n):
            if l[min_id] > l[j]:
                min_id=j
        # swap the minimum element with element next to sorted sub-array
        if i!=min_id:
            l[i],l[min_id] =l[min_id],l[i]
            # not stable because btw i & min_id may have same element as l[i] 
            


# Time: best/avg/worst: O(n^2) ; Not Stable; In-place Algorithm as O(1) Aux space
# Does less memory writes than merge, quick, insertion sort..
# But Cycle sort is optimal in terms of memory writes

# it is basic idea for heap sort
# heap sort idea is based on selection sort only but it uses heap data structure to optimize this idea

# The selection sort algorithm sorts an array by repeatedly finding the minimum element (considering ascending order) from unsorted part and putting it at the beginning. 
# The algorithm maintains two subarrays in a given array.
# 1) The subarray which is already sorted.
# 2) Remaining subarray which is unsorted. 
#   In every iteration of selection sort, the minimum element (considering ascending order) 
#   from the unsorted subarray is picked and moved to the sorted subarray.



if __name__ == "__main__":
    l = [9,2,5,15,9,3,11,1] 
    selectionSort(l)
    print(l)