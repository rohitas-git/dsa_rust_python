from paritionLomutoScheme import lomutoParition as parition

def qSort(arr,l,h):
    if l < h:
        p = parition(arr,l,h)    
        # print("after parition",arr[l:h+1])
        qSort(arr,l,p-1)
        # print("first qSort",arr[l:p])
        qSort(arr,p+1,h)
        # print("Second qSort",arr[p+1:h+1])

    
if __name__ == "__main__":
    arr = [8,4,7,9,3,10,5]
    qSort(arr,0,6)
    print(arr)