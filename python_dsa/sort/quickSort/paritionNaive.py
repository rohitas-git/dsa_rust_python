# It will be used as sub-routine of quickSort algo

# time - theta(3*N), space - theta(N)
# Stable Algorithm
def naiveSolution(arr, index):
    n = len(arr)
    arr[index],arr[n-1] = arr[n-1], arr[index]
    tmp = []
    
    for x in arr:
        if x <= arr[n-1]:
            tmp.append(x)
    for x in arr:
        if x > arr[n-1]:
            tmp.append(x)
    
    for i in range(n):
        arr[i] = tmp[i]
    
    print(arr)        
    

if __name__ == "__main__":
    a = [3,1,12,15,10,8,9,21]
    id = 4
    
    naiveSolution(a,id)