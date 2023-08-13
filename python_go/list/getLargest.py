
# t: theta(n)
def getLargest(l):
    
    if not l:
        return None
    else:
        res = l[0]
        for i in range(1,len(l)):
            if l[i] > res:
                res = l[i]
        return res 


def getSecondLargest(l):
    if len(l) == 1:
        return None
    
    lar = getLargest(l)
    slar = None
    for x in l:
        if x != lar:
            if slar == None:
                slar = x
            else:
                slar = max(slar,x) 
    return slar



# Approach: Find the second largest element in a single traversal. 
# Below is the complete algorithm for doing this:  
# 1) Initialize the first as 0(i.e, index of arr[0] element
# 2) Start traversing the array from array[1],
#    a) If the current element in array say arr[i] is greater
#       than first. Then update first and second as,
#       second = first
#       first = arr[i]
#    b) If the current element is in between first and second,
#       then update second to store the value of current variable as
#       second = arr[i]
# 3) Return the value stored in second.
def efficientSecLargest(l):
    if len(l) <= 1:
        return None
    
    lar = l[0]
    slar = None
    
    for x in l[1:]:
        if x > lar:
            slar = lar
            lar = x
        elif x != lar:
            if slar == None or x > slar:
                slar = x
    
    return slar 
            
    
l = [ 12,1,23,14,99,56]
print(getLargest(l))
print(efficientSecLargest(l))
