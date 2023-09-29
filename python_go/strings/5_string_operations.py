# String Operation

# ---------------------Substring--------------------
s1 = "geeksforgeeks"
s2 = "geeks"  
# all consecutive chars are substring of a given string
# substring in a string: T/F
print(s2 in s1)
print(s2 not in s1)

# ---------------------Concatenation--------------------
# one string is appended at end of another string
s1 = "going "
s2 = "home"
print(s1+s2)

# ---------------------Finding position of substring--------------------
s1 = "geeks|geeks|geeks"
s2 = "geeks"  
print(s1.index(s2)) # find first occurence else raise value error
print(s1.index(s2,1,13))
print(s1.rindex(s2)) # find last occurence else raise value error

s1 = "geeks geeks geeks"
s2 = "geeks"  
print(s1.find(s2)) # returns index of first occurence else
print(s1.find("gfg")) # doesn't raise error but returns -1

# ---------------------Others--------------------
s = "GeeK"
print(len(s))
print(s.lower())
print(s.upper())
print(s)
print(s.islower())
print(s.isupper())

print(s.startswith("Ge"))
print(s.endswith("eK"))
print(s.startswith("GeeK",0,len(s)))

print("geeks\n  for\n  geeks".split())
print("geeks   for   geeks".split())
l = "geeks, for, geeks".split(", ")
print("-".join(l))


print("--geeksforgeeks---".strip("-"))
print("--geeksforgeeks---".lstrip("-"))
print("--geeksforgeeks---".rstrip("-"))
print("   geeksforgeeks  \n ".strip())




