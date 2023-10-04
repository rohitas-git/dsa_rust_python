
    
# Python program to find union of
# two sorted arrays
# Function prints union of arr1[] and arr2[]
# m is the number of elements in arr1[]
# n is the number of elements in arr2[]
# Time Complexity : O(m + n)
# Auxiliary Space: O(1)
def printUnion(arr1, arr2, m, n):
    i, j = 0, 0
    while i < m and j < n:
        if arr1[i] < arr2[j]:
            print(arr1[i],end=" ")
            i += 1
        elif arr2[j] < arr1[i]:
            print(arr2[j],end=" ")
            j+= 1
        else:
            print(arr2[j],end=" ")
            j += 1
            i += 1
 
    # Print remaining elements of the larger array
    while i < m:
        print(arr1[i],end=" ")
        i += 1
 
    while j < n:
        print(arr2[j],end=" ")
        j += 1
 

# Handling duplicates in any of the arrays: 
# Above code does not handle duplicates in any of the arrays. 
# To handle the duplicates, just check for every element whether adjacent elements are equal. 
# Python3 program to find union of two
# sorted arrays (Handling Duplicates)
# Time Complexity : O(m + n)
# Auxiliary Space: O(1)
def union_array(arr1, arr2):
    m = len(arr1)
    n = len(arr2)
    i = 0
    j = 0
     
    # keep track of last element to avoid duplicates
    prev = None
     
    while i < m and j < n:
        if arr1[i] < arr2[j]:
            if arr1[i] != prev:
                print(arr1[i], end=' ')
                prev = arr1[i]
            i += 1
        elif arr1[i] > arr2[j]:
            if arr2[j] != prev:
                print(arr2[j], end=' ')
                prev = arr2[j]
            j += 1
        else:
            if arr1[i] != prev:
                print(arr1[i], end=' ')
                prev = arr1[i]
            i += 1
            j += 1
             
    while i < m:
        if arr1[i] != prev:
            print(arr1[i], end=' ')
            prev = arr1[i]
        i += 1
 
    while j < n:
        if arr2[j] != prev:
            print(arr2[j], end=' ')
            prev = arr2[j]
        j += 1


# Time Compleixty : O((m+n)log(m+n))
# Space Complexity : 
# O(m+n) {If Space of Union ArrayList is considered} 
# O(1) {If Space of union ArrayList is not considered}
def find_union_v1(arr1, arr2):
    freq = {}
    union = []
    
    for num in arr1:
        freq[num] = freq.get(num, 0) + 1
    
    for num in arr2:
        freq[num] = freq.get(num, 0) + 1
    
    for num in freq:
        union.append(num)
    
    return union

arr1 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
arr2 = [2, 3, 4, 4, 5, 11, 12]

union = find_union_v1(arr1, arr2)

print("Union of arr1 and arr2 is:")
for num in union:
    print(num, end=" ")
print()



# Time Compleixty : O( (m+n)log(m+n) 
# 
# Space Complexity : 
# O(m+n) {If Space of Union ArrayList is considered} 
# O(1) {If Space of union ArrayList is not considered}
def find_union_v2(arr1, arr2):
    s = set()
    union = []
    
    for num in arr1:
        s.add(num)
    
    for num in arr2:
        s.add(num)
    
    for num in s:
        union.append(num)
    
    return union


# Time Complexity: O(m*log(m) + n*log(n)) where ‘m’ and ‘n’ are the size of the arrays
# Auxiliary Space: O(m + n)
# Python code to implement the approach

def Unionarray(arr1, arr2, n, m):
	# Create a set to store unique elements from both arrays
	set1 = set(arr1)
	set2 = set(arr2)
	# Merge both sets and convert back to list
	result = list(set1.union(set2))
	return result

# Driver code
if __name__ == "__main__":
	arr1 = [1, 2, 2, 2, 3]
	arr2 = [2, 3, 3, 4, 5, 5]
	n = len(arr1)
	m = len(arr2)

	# Function call
	uni = Unionarray(arr1, arr2, n, m)
	for i in uni:
		print(i, end=" ")

