def getLastOccurrenceIterative(arr,x):   
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
            if mid == len(arr) -1  or arr[mid] != arr[mid+1]:
                return mid
            else:
                low = mid + 1
                
        
            
    return -1



def getLastOccurrenceRecursive(arr,x, low, high):
    if x < arr[low]:
        return -1
    if x > arr[high]:
        return -1
    
    if low > high:
        return -1
    
    mid = low + (high-low)//2
    
    if x > arr[mid]:
        return getLastOccurrenceRecursive(arr,x,mid+1,high)
    elif x < arr[mid]:
        return getLastOccurrenceRecursive(arr,x,low, mid-1)
    else:
        if mid == len(arr) -1  or arr[mid] != arr[mid-1]:
            return mid
        else:
            return getLastOccurrenceRecursive(arr,x,mid+1, high)
        
    # another code for above control flow
        # if ((mid == len(arr) -1 or x < arr[mid+1]) and arr[mid] == x):
        #     return mid
        # elif (x > arr[mid]):
        #     return getFirstOccurrenceIterative(arr, x, (mid + 1), high)
        # else:
        #     return getFirstOccurrenceIterative(arr, x, low, (mid -1))