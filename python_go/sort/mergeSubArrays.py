def main():
    l = [5,6,20,6,10,16,30]
    l3 = mergeSubarrays(l,0,2,6)


# low <= mid < high
# time - theta(m+n) 
# space - theta(m+n) 
def mergeSubarrays(l, low, mid, high):
    # aux space - O(m+n)
    l1 = l[low:mid+1]
    l2 = l[mid+1:high+1]
    
    n = len(l1)
    m = len(l2)    
    
    i,j, = 0, 0
    k = low
    
    while i < n and j < m:
        # <= instead of < to have stability by prefering left over right when =
        if l1[i] <= l2[j]:
            l[k] = l1[i]
            k+=1
            i+=1
        else: 
            l[k] = l2[j]
            k+=1
            j+=1
        
    while i<n:
        l[k] = l1[i]
        k+=1
        i+=1
    
    while j<m:
        l[k] = l2[j]
        k+=1
        j+=1
    
    print(l)    


main()