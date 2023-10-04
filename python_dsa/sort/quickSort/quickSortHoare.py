from paritionHoare import hoareParition as parition

# In hoare, we don't have exact position of pivot after parition
# hoare parition will return index of end of first part (<= Pivot parition )
# [<= Pivot] [>=Pivot]

def qSort(arr,l,h):
    if l < h:
        p = parition(arr,l,h) 
        qSort(arr,l,p)
        qSort(arr,p+1,h)
        


if __name__ == "__main__":
    # arr = [8,4,7,9,3,10,5]
    arr = [10,110,30,90,20,50]
    # qSort(arr,0,6)
    # qSort(arr,0,5)
    # print(arr)
    print(parition(arr,0,5))