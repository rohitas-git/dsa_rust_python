#------------------------Removal----------------------------------
s = {10,30,20,40}

# doesn't do anything if value is not present in set
s.discard(30)
print(s)

# raise an error if value is not present in set
s.remove(20)
print(s)

# clears set to an empty set
s.clear()
print(s)
s.add(50)
 
# del removes the object itself (can't access deleted object afterwards)
del s 
