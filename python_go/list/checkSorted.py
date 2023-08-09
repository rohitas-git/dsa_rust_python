# Check if List is sorted in Non-decreasing order

# Method 1 > 2, more efficient 

# Method 1
def checkSorted(l):
    if len(l) == 1 or len(l)==0 or not l:
        return True
    
    i = 1
    while i < len(l):
        if l[i] < l[i-1]:
            return False
        i+=1
    return True
    

# Method 2
def isSorted(l):
    sl = sorted(l)
    print(sl)
    print(l)
    if sl == l:
        return True
    else:
        return False

        
        
l1 = [10,20,30]
l2 = []
l3 = [1]
l4 = [10,10]
l5= [10,2,1,14]

# print(checkSorted(l1))
# print(checkSorted(l2))
# print(checkSorted(l3))
# print(checkSorted(l4))
# print(checkSorted(l5))

# l5.sort()
# print(checkSorted(l5))


    
print(isSorted(l5))
# isSorted(l1)