
# * STRING in Python
# - Sequence of characters 
# - Used to store text data
# - typically small set of characters
# - Characters 'A' to 'Z' are stored as values from [65,90]
# - Characters 'a' to 'z' are stored as values from [97,122]

# Chars are internally stored as integer
# ! No separate type for characters in PYTHON
# 'A' -> string with one char  

# ASCII - 128 chars
# Extended ASCII - 256 chars 
# Unicode allows chars from multiple langs like Hindi, Chinese

# Chars in english lang (english chars, &$#@ etc) - total 128
# They have same values in both unicode and ascii

#----------------------- Char <> Int --------------------------

print("a:\t ", ord('a'))
print("A:\t",ord('A'))
print("97:\t",chr(97))
print("65:\t",chr(65))

