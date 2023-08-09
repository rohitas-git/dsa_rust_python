

# arr = [10,20,20,30,30,30] -> arr = [10,20,30]
# not working for [10,20,30, 30,_20, _10, _30]
def removeDuplicates(arr,n):
    
    res = 1
    for i in range(1,n):
        if arr[res-1] != arr[i]:
            arr[res]=arr[i]
            res+=1
    return res



# time  theta(n) + O(n) = O(n)
# space O(n)
def naive(l,n):
    
    tmp = []*n
    
    # Copying because for any array, first element will always be distinct
    tmp[0] = l[0]
    
    res = 1
    
    for i in range(1,n):
        if tmp[res-1] != l[i]:
            tmp[res] = l[i]
            res+=1
    for i in range(0,res):
        l[i] = tmp[i]
    
    return res