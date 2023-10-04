

def main():
    l = [x for x in range(1,11)]
    
    even, odd = getEvenOdd(l)  



# Time complexity: O(n)
# Space complexity: O(1)
def getEvenOdd(l):
    # even =[]
    # odd = []
    
    # for x in l :
    #     if x%2 == 0:
    #         even.append(x)
    #     else:
    #         odd.append(x)
    
    even =  [x for x in l if x%2 ==0 ]
    odd =  [x for x in l if x%2 !=0 ]
    
    print(f"Odd: {odd}")
    print(f"Even: {even}")
    return even, odd

main()