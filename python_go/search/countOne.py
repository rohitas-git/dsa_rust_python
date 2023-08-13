# Count 1's in a Sorted Binary List

from firstOccurrence import getFirstOccurrenceIterative

# Count 1's in a Sorted Non-Decreasing Binary List - [0,0,1,1,1,1]
def countOnes(arr):
    first = getFirstOccurrenceIterative(arr,1)
    if first == -1:
        return 0
    last = len(arr) - 1
    total = last - first + 1
    return total


# Count 1's in a Sorted Non-Increasing Binary List - [1,1,1,0,0,0,0]
def countOnes(arr):
        N = len(arr)
        low=0
        high=N-1
        
        while low<=high:
            mid = (low+high)//2
            curr=arr[mid]
            
            if (mid == N-1 or arr[mid+1] == 0) and curr == 1:
                return mid+1
            elif curr == 1:
                low = mid+1
            elif curr < 1:
                high = mid-1
        
        return 0
    
    
# Returns counts of 1's in arr[low..high].  The array is
# assumed to be sorted in non-increasing order

# Time complexity: O(Log(N))
# Auxiliary Space: O(log(N))
def countOnes(arr, low, high):

    if high >= low:

        # get the middle index
        mid = low + (high-low)//2

        # check if the element at middle index is last 1
        if ((mid == high or arr[mid+1] == 0) and (arr[mid] == 1)):
            return mid+1

        # If element is not last 1, recur for right side
        if arr[mid] == 1:
            return countOnes(arr, (mid+1), high)

        # else recur for left side
        return countOnes(arr, low, mid-1)

    return 0



def countOnes(arr, n):
    low = 0
    high = n - 1
    while (low <= high):  # get the middle index
        mid = (low + high) // 2

        # else recur for left side
        if (arr[mid] < 1):
            high = mid - 1

        # If element is not last 1, recur for right side
        elif(arr[mid] > 1):
            low = mid + 1
        else:

            # check if the element at middle index is last 1
            if (mid == n - 1 or arr[mid + 1] != 1):
                return mid + 1
            else:
                low = mid + 1

    return 0