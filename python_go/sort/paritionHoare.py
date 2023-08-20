# Hoare's parition Scheme
# In this, we consider the first element as pivot

# - It doesn't fix the pivot in contrast to Lomuto scheme 
#       where pivot is fixed at last element during process and at end, we also get pivot position
# - In genral, Hoare > Lomuto parition scheme
# - Both are not Stable algorithm except Naive algorithm for parition

# [< Pivot][Unknown??][>= Pivot] 
# l________i_________j___________h 
#          -->      <--

# It feels like we are position the pivot from both direction 
# by making use of rule that leftItem < pivot and rightItem > pivot
# On the way, if we violation of this, we swap them for correction
# Repeat till both direction ptrs cross each other



# time - O(n)
# space - O(1)
def hoareParition(arr,l,h):
    
    pivot = arr[l]
    i = l
    j = h
    
    while True:
        while arr[i] < pivot:
            i+=1
        while arr[j] > pivot:
            j-=1
        if i>=j:
            return j # pivot will not at j but somewhere in [j+1:h]
        arr[j],arr[i] = arr[i], arr[j]
        i+=1
        j-=1



if __name__ == "__main__":
    # arr = [40,20,50,80,30,90]
    arr = [11,1,12,15,10,8,9,21]
    print(hoareParition(arr,0,7))
    print(arr)
    
    cornerCase1 = [5,5,5,5]
    cornerCase2 = [1,2,13,8,10]
    cornerCase3 = [2,5,1,7,4,15]