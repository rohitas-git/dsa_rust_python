# ---------------------------- Demonstrate Removal --------------------------- #

l = [ (x * 10) for x in range(1,8)]

l.remove(20)
print(l)

print(l.pop())
print(l)

print(l.pop(2))
print(l)

del l[1]
del l[0:2]
print(l)

# Complexities for Deleting elements in a Lists(pop() method):
# Time Complexity: O(1)/O(n) (O(1) for removing the last element, O(n) for removing the first and middle elements)
# Space Complexity: O(1)