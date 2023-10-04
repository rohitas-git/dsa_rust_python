def bubbleSort(l):
    n = len(l)
    
    for i in range(n-1):
        done = True
        for j in range(n-i-1):
            if l[j] > l[j+1]:
                l[j],l[j+1] = l[j+1],l[j]
                done = False
        if done:
            return
                
                

# time: avg/worst: O(n^2), best: O(n)
# Stable: adjacent same elements are not swapped => Order/Stability is preserved
# Only used in academic: How would you sort the array by swapping adjacent data?
# Not practical usage of Bubble sort, 
# heterogeneous algorithms are used for practical application

# Bubble Sort is the simplest sorting algorithm that works by 
# repeatedly swapping the adjacent elements if they are in the wrong order. 
# This algorithm is not suitable for large data sets as 
# its average and worst-case time complexity is quite high.

if __name__ == "__main__":
    l = [2,5,15,9,3,11,8]
    bubbleSort(l)
    print(l)