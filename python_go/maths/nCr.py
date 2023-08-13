# You are given two numbers n and r. You need to find nCr.
# nCr = (n!) / ((n-r)!*(r!))
# In recursive way, we can write nCr as nCr = (n-1)C(r-1) + (n-1)Cr

# Expected Time Complexity: O(2N).
# Expected Auxiliary Space: O(2N) (Recursive).
def nCr(n,r):
    #code here
    if n == r:
        return 1
    if r == 0:
        return 1
    return nCr(n-1,r-1) + nCr(n-1,r)