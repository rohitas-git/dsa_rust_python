# Operations with two sets

s1 = {2,4,6,8}
s2 = {3,6,9}
print("s1",s1)
print("s2",s2)

# Union 
print("Union",s1 | s2) 

# Intersection
print("Intersection",s1 & s2) 

# Difference
print("s1-s2",s1 - s2 )
print("s2-s1",s2 - s1 )
print("s1-s1",s1 - s1 )

# Symmetric Difference
print("Symmetric Difference",s1^s2)
