# Parition our array around the pivot element
# In Lomuto Parition Scheme, pivot is last element

# Parition the array according to a given parition point (pivot)
# such that elements smaller than ppValue are to left it
# and larger elements to right of it

# Example:
# arr = [10,50,30,90,20,110] and parition point = 1
# result: arr = [10,30,20, 50 ,110,90]

# Lomuto Parition:
# Takes last element as pivot (any pivot is possible; here it's used for simplicity)
# Will be used in quicksort recursively, therefore have l and h in fn args
# - Unstable algorithm

# [< Pivot][>= Pivot][Unprocessed] 
# l_______i_________j___________h 

# element at i+1 is >= pivot (first element of >= parition)
# element at j is unprocessed 

def lomutoParition(arr,l,h):
    
    pivot = arr[h]
    i = l-1
    
    # for smaller elements, swap them with first elements of (>=) Parition 
    # and increase ptr to last elemenet of (<) +1
    # for larger elements, there is already fit for(>=) Parition; +1 the ptr to its last element
    for j in range(l,h):
        if arr[j] < pivot:
            i= i+1
            arr[j],arr[i] = arr[i],arr[j]
            
    # swap the pivot to correct position
    arr[h], arr[i+1] = arr[i+1],arr[h]
    
    return i+1

if __name__ == "__main__":
    arr = [10,20,50,110,30]
    print(lomutoParition(arr,0,4))
    print(arr)