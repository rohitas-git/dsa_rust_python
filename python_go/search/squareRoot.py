
# returns floor of square root of x
# time O(logN) 
def findSquareRoot(x):
    low = 1
    high = x
    ans = -1
    
    while low <= high:
        mid = (low + high)//2
        mSq = mid * mid
        
        if mSq == x:
            return mid
        elif mSq > x:
            high = mid-1
        else:
            low = mid + 1
            ans = mid
    
    
    pass
    
    
    
# time O( sqrt(N) / x^(0.5) )
def naive(x):
    i = 1
    while i*i <= x:
        i+=1
    return i-1
    