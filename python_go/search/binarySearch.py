# Binary Search - Optimal searching algorithm for a sorted array
# To find a number in the array


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
    b = isSorted(arr)
    n = len(arr)
    if not b :
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
print(linearSearch(arr,1))


def binarySearchRecursive(arr,num, low, high):
    if not isSorted(arr) :
        return -1
    if num < arr[low]:
        return -1
    if num > arr[high]:
        return -1

    n = len(arr)
    mid = (low+high)//2
    midValue = arr[mid]
    if midValue == num:
        return mid
    elif num > midValue:
        return binarySearchRecursive(arr,num, mid+1,high)
    else:
        return binarySearchRecursive(arr, num, low,mid-1) 
        

def binarySearchIterative(arr,num):
    low = 0
    high = len(arr) - 1
    
    while low <= high:
        pass
        

arr= [10,20,30,40,50,100,110]
print(binarySearchRecursive(arr,50, 0,len(arr)-1))