# Given a sorted array arr[] of size N without duplicates, and given a value x. 
# Floor of x is defined as the largest element K in arr[] such that K is smaller than or equal to x. 
# Find the index of K(0-based indexing).

def findFloor(arr,N,X):
        #Your code here
        low=0
        high=N-1
        kId = -1
        
        while low<= high:
            mid = (low+high)//2
            curr =arr[mid]
            
            if curr == X:
                return mid    
            if curr < X:
                if kId == -1 or arr[kId] < curr:
                    kId = mid
                low = mid+1
            else:
                high = mid-1
        return kId