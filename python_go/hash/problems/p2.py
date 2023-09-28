 #Function to return non-repeated elements in the array.
def printNonRepeated(arr,n):
    d = {}
    for i in range(n):
        if arr[i] not in d:
            d[arr[i]] = 1
        else:
            d[arr[i]] += 1
        
    non_repeated_elements = [item for item in arr if d[item]== 1]
    # print(non_repeated_elements)
    return non_repeated_elements


        

if __name__ == "__main__":
    n = 11
    arr = [1, 1, 2, 2, 3, 3, 3, 4, 5, 6, 7]
    printNonRepeated(arr,n)   
    
# end main