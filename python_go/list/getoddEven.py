

def main():
    l = [10, 2, 3, 8 ,7]
    l2 = [10,30,40]  
    
    even, odd = getEvenOdd(l)  



# Time complexity: O(n)
# Space complexity: O(1)
def getEvenOdd(l):
    even =[]
    odd = []
    
    for x in l :
        if x%2 == 0:
            even.append(x)
        else:
            odd.append(x)
    
    # print(f"Odd: {odd}")
    # print(f"Even: {even}")
    return even, odd

main()