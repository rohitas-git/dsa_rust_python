# Inversion:

# Inversion Count for an array indicates 
# – how far (or close) the array is from being sorted. 
# If the array is already sorted, then the inversion count is 0, 
# but if the array is sorted in reverse order, the inversion count is the maximum.

# A pair a[i], a[j] form an inversion 
# when i < j and a[i] > a[j]


# Average case Time - Theta(N*logN)
# Average case Space - O(N)
def inversion(arr, l, r):    
    # if specialCases(a) != None:
    #     return specialCases

    res = 0
    if l < r:
        m = (l+r)//2
        res += inversion(arr,l,m)
        res += inversion(arr,m+1,r)
        res += merge(arr,l,m,r)
    return res


def merge(arr,l,m,r):
    left = arr[l:m+1]
    right = arr[m+1:r+1]
    res, k, i, j = 0, l, 0 ,0
    while i < len(left) and j < len(right):
        if left[i] <= right[j]:
            arr[k] = left[i]
            i+=1
        else:
            arr[k] = right[j]
            j+=1
            res += (len(left) - i)
        k+=1
    while i < len(left):
        arr[k] = left[i]
        k+=1
        i+=1
    while j < len(right):
        arr[k] = right[j]
        k+=1
        j+=1
    
    return res

def specialCases(a):
    n = len(a)
    
    ascSorted = True
    dscSorted = True
    
    for i in range(n-1):
        if a[i] <= a[i+1]:
            dscSorted = False
            break
        if a[i] >= a[i+1]:
            ascSorted = False
            break
    
    if ascSorted:
        print(0)
        return 0 
    
    if dscSorted:
        t = n*(n-1)/2
        print(t)
        return t
    
    return None



def naive(a):
    n = len(a)
    res= 0
    for i in range(n-1):
        for j in range(i+1,n):
            if a[i] > a[j]:
                res+=1
    print(res)
    return res

if __name__ == "__main__":
    a = [2,4,1,3,5]
    print(inversion(a,0,len(a)-1))
    # naive(a)

    b = [10,20,30,40]
    print(inversion(b,0,len(b)-1))
    
    c = [40,30,20,10]
    print(inversion(c,0,len(c)-1))