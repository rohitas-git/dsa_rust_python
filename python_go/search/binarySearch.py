# Binary Search - Optimal searching algorithm for a sorted array
# To find a number in the array

# For binary search recursive:
# T(n) = T(n/2) + O(1)


def isSorted(arr):
    if len(arr) == 0 or len== 1:
        return False
    asc = True
    dsc = True
    for i in range(len(arr)-1):
        if arr[i] <= arr[i+1]:
            dsc= False
        else:
            asc=False
    if asc or dsc:
        return True    
        
    
# arr= [10,20,30,40]
# print(isSorted(arr))

# time: O(n), space O(n)
def linearSearch(arr, num):
    n = len(arr)
    
    if not isSorted(arr) :
        return -1
    if num < arr[0]:
        return -1
    if num > arr[n-1]:
        return -1
    
    for i in range(n):
        if arr[i] == num:
            return i
    return -1
    

arr= [10,20,30,40]
print(linearSearch(arr,20))


# Time O(logN), Space O(logN)
# O(n) for Successful Searches
# Theta(1) for Unsuccessful Searches due to optimization
def callBinarySearchRecursive(arr,num):
    return binarySearchRecursive(arr,num,0, len(arr)-1)

def binarySearchRecursive(arr,num, low, high):
    if not isSorted(arr) :
        return -1
    # optional; to optimize the search: Theta(1) for Unsuccessful Searches 
    if num < arr[low]:
        return -1
    if num > arr[high]:
        return -1

    n = len(arr)
    mid = low + (high-low)//2
    midValue = arr[mid]
    
    if midValue == num:
        return mid
    
    elif num > midValue:
        return binarySearchRecursive(arr,num, mid+1,high)
    
    else:
        return binarySearchRecursive(arr, num, low,mid-1) 
        

# Time O(logN), Space O(1)
# O(n) for Successful Searches
# Theta(logN) for Unsuccessful Searches
def binarySearchIterative(arr,num):
    low = 0
    high = len(arr) - 1
    
    while low <= high:
        mid = low + (high-low)//2
        
        if arr[mid] == num:
            return mid
        elif arr[mid] < num:
            low = mid + 1
        else:
            high = mid - 1
    return -1

arr= [10,20,30,40,50,100,110]
print(binarySearchRecursive(arr,20, 0, len(arr)-1))
print(callBinarySearchRecursive(arr,20))
print(binarySearchIterative(arr,30))


# Why use mid = low + (high – low)/2 ?
# Why not use mid = (low + high)/2 ?
# Because sum of low and high can cause bugs due to overflow issue
# Therefore, better to use mid = low + (high-low)/2