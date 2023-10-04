# Insertion Sort
# - O(n^2) worst case & O(n) for best case
# - In-place and Stable 
# - preferred to be used in practice for small arrays (TimSort and IntroSort)

def insertionSort(arr):
    n = len(arr)
    
    for i in range(1,n):
        curr = arr[i]
        j = i-1
        
        # for sorted and almost sorted arrays, it will enter inner loop less times
        while j >= 0 and curr < arr[j]:
            arr[j+1] = arr[j]
            j-=1
        arr[j+1] = curr
    
    print(arr)
    
# Insertion sort is a simple sorting algorithm 
# that works similar to the way you sort playing cards in your hands. 
# The array is virtually split into a sorted and an unsorted part. 
# Values from the unsorted part are picked and placed at the correct position in the sorted part.
    
if __name__ == "__main__":
    arr = [20,5,40,60,10,30]
    insertionSort(arr)