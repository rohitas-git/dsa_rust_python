# ----------------------- Frequencies of Array Elements ---------------------- #
# Given an array which may contain duplicates, 
# print all elements and their frequencies.


def countFreqOptimalV2(arr, n):
    
    mp = {}
    
    # Traverse through array elements and
    # count frequencies
    for i in range(n):
        if arr[i] not in mp:
            mp[arr[i]] = 0
        mp[arr[i]] += 1
        
    # To print elements according to first
    # occurrence, traverse array one more time
    # print frequencies of elements and mark
    # frequencies as -1 so that same element
    # is not printed multiple times.
    for i in range(n):
        if (mp[arr[i]] != -1):
            print(arr[i],mp[arr[i]])
        mp[arr[i]] = -1


# Time Complexity : O(n) 
# Auxiliary Space : O(n)
def countFreqOptimal(arr, n):

    mp = dict()

    # Traverse through array elements 
    # and count frequencies
    for i in range(n):
        if arr[i] in mp.keys():
            mp[arr[i]] += 1
        else:
            mp[arr[i]] = 1
            
    # Traverse through map and print 
    # frequencies
    for x in mp:
        print(x, " ", mp[x])


# Time Complexity : O(n2) 
# Auxiliary Space : O(n)
def countFreqNaive(arr, n):
    
    # Mark all array elements as not visited
    visited = [False for i in range(n)]

    # Traverse through array elements 
    # and count frequencies
    for i in range(n):
        
        # Skip this element if already 
        # processed
        if (visited[i] == True):
            continue

        # Count frequency
        count = 1
        for j in range(i + 1, n, 1):
            if (arr[i] == arr[j]):
                visited[j] = True
                count += 1
        
        print(arr[i], count)