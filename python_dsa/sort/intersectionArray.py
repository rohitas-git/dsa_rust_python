
def intersection(a,b):
    n = len(a)
    m = len(b)
    
    res = []
    
    i, j = 0, 0
    
    while i < n and j < m:
        if i> 0 and a[i-1] == a[i]:
            i+=1
        if j>0 and b[j-1] == b[j]:
            j+=1
        
        if a[i] < b[j]:
            i+=1
        elif a[i] > b[j]:
            j+=1
        else:
            res.append(a[i])
            i+=1
            j+=1
        
    print(res)



if __name__ == "__main__":
    a = [3,5,10,10,10,15,15,20]
    b = [5,10,10,15,30]
    
    # 5 10 15
    intersection(a,b)