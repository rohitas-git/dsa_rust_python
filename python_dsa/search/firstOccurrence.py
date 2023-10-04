
def getFirstOccurrenceIterative(arr,x):   
    low = 0
    high = len(arr) - 1
    
    if x < arr[low]:
        return -1
    if x > arr[high]:
        return -1
    
    while low <= high:
        mid = low + (high-low)//2
        
        if arr[mid] < x :
            low = mid + 1
        elif arr[mid] > x:
            high = mid - 1
        else:
            if mid == 0 or arr[mid-1] != arr[mid]:
                return mid
            else:
                high = mid -1
                
        # another code for above control flow
        # while low<=high:
        #     mid = (low+high)//2
        #     curr=arr[mid]
        #     if (mid == N-1 or arr[mid+1] != X) and curr == X:
        #         return mid
        #     elif curr > X:
        #         high = mid-1
        #     else:
        #         low = mid+1
        # return -1
            
    return -1



def getFirstOccurrenceRecursive(arr,x, low, high):
    if x < arr[low]:
        return -1
    if x > arr[high]:
        return -1
    
    if low <= high:
        mid = low + (high-low)//2
        if x > arr[mid]:
            return getFirstOccurrenceRecursive(arr,x,mid+1,high)
        elif x < arr[mid]:
            return getFirstOccurrenceRecursive(arr,x,low, mid-1)
        else:
            if mid == 0 or arr[mid] != arr[mid-1]:
                return mid
            else:
                return getFirstOccurrenceRecursive(arr,x,low, mid-1)
        
    return -1
    

# arr = [10,15,20,20,30,40,40,40]

# print(getFirstOccurrenceIterative(arr,20))
# print(getFirstOccurrenceIterative(arr,40))

# print(getFirstOccurrenceRecursive(arr,20,0,7))
# print(getFirstOccurrenceRecursive(arr,40,0,7))
