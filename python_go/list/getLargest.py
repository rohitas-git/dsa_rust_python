
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


#theta(N) but more efficient as it does only 1 traversal of list 
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
