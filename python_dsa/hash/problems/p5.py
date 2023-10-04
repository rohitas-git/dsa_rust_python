
def main(): 
    s = "7 4 0 9 4 8 8 2 4 5 5 1"
    arr = [ int(i) for i in s.split(" ")]
    n = len(arr)
    print(firstRepeated(arr,n))
    
    s = "1 5 3 4 3 5 6"
    arr = [ int(i) for i in s.split(" ")]
    n = len(arr)
    print(firstRepeated(arr,n))


#Function to return the position of the first repeating element.
def firstRepeated(arr, n):
    firstOccurenceDic ={}
    isRepeated ={}
    
    for i in range(n):
        if arr[i] not in firstOccurenceDic:
            firstOccurenceDic[arr[i]] = i
            isRepeated[i]=False
        else:
            isRepeated[firstOccurenceDic[arr[i]]]=True
    
    print(firstOccurenceDic)
    print(isRepeated)
    
    for id in isRepeated.items():
        if id[1] == True:
            return id[0]
    return -1
     
    
    


main()