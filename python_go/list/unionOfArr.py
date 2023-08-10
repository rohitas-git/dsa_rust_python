


# Time Compleixty : O( (m+n)log(m+n) 
# 
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

arr1 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
arr2 = [2, 3, 4, 4, 5, 11, 12]

union = find_union_v2(arr1, arr2)

print("Union of arr1 and arr2 is:")
print(*union)


