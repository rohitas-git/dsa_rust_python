# SETS
# - Distrinct elements
# - Unordered
# - No indexing
# !- Union, Intersection, Difference etc. are fast
# - Uses hashing internally (=> fast union, diff ... etc)


s1 = {10,20,30}
print(s1)

s2= set([20,30,40])
print(s2)

s3={}
print(type(s3))

s4= set()
print(type(s4))
print(s4)


#-------------------------Other operations------------------------------------

s = {10,20,30}
print(len(s))
print(20 in s) 
print(50 in s) 