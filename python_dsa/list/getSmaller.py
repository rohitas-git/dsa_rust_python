

def main():
    l = [x for x in range(1,20)]
    
    smaller = getElementsSmallerThanV2(10, l)  
    print(smaller)



# Time complexity: O(n)
# Space complexity: O(1)
def getElementsSmallerThan(s, l):
    smaller =[]
    
    for x in l :
        if x < s:
            smaller.append(x)

    return smaller

def getElementsSmallerThanV2(s, l):
    smaller = [x for x in l if x < s]
    
    return smaller
    
    
main()