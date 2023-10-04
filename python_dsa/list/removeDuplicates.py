 
def removeDuplicates(arr):
    n = len(arr)
    
    if n==0 or n==1:
        return n
    
    overrider = 0
    
    for ptr in range(0,n-1):
        if arr[ptr] != arr[ptr+1]:
            arr[overrider] = arr[ptr]
            overrider+=1
    
    arr[overrider] = arr[n-1]
    
    overrider+=1
    return overrider

# time: O(n), space O(1)

# Driver code
arr = [1, 2, 2, 3, 4, 4, 4, 5, 5]

# removeDuplicates() returns
# new size of array.
n = removeDuplicates(arr)

# Print updated array
for i in range(0, n):
    print ("%d"%(arr[i]), end = " ")