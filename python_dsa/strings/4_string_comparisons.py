
s1 = "geeksforgeeks"
s2 = "ide"

# Compare two strings: compare character by character based on their unicodes 

print(s1 < s2) # True as ord('g') < ord('i') 
print(s1 <= s2)
print(s1 > s2)
print(s1 >= s2)
print(s1 == s2) # True iff both strings have same values
print(s1 != s2)

# comparisons are True: 
"abcd" > "abc"
"ZAB" > "ABC"
"abc" > "ABC"
"x" > "abc"