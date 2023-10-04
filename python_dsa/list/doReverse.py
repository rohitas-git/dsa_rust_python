
# reverse given list
def reverseV1():
    l = [10,20,30]
    l.reverse()
    print(l)


# return new reversed list
def reverseV2():
    l = [10,20,30]
    l = l[::-1]
    print(l)


# return new reversed list
def reverseV3():
    l = [10,20,30]
    new = list(reversed(l))
    print(new)
    print(l)

# 2 > 1 > 3

# ReverseV1 only present on list as they are mutable 
# Not on tuple and string

# V2 V3 work for tuple string also

# time: O(N/2) = O(N) 
def SwapFrontBack():
    l = [10,20,30,40,50,60]
    print(l)
    t = len(l)
    for i in range(0, t//2):
        if i != t -1 -i:
            (l[i],l[t-1-i])=(l[t-1-i],l[i])
    
    print(l) 


SwapFrontBack()   



