

def main():
    l = [x for x in range(1,20)]
    
    smaller = getElementsSmallerThan(10, l)  
    print(smaller)



# Time complexity: O(n)
# Space complexity: O(1)
def getElementsSmallerThan(s, l):
    smaller =[]
    
    for x in l :
        if x < s:
            smaller.append(x)
    
    # print(f"Odd: {odd}")
    # print(f"Even: {even}")
    return smaller

main()