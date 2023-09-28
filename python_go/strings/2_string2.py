
# 1. Char strings <> Int internally
# 2. String are immutable in PYTHON
# 3. Multi-line strings
# 4. Escape Characters 
# 5. Raw strings

# 2. 
s = "geek"
# s[0] = "e" # TypeError: 'str' object does not support item assignment
print(s) 

# 3.  """ OR ''' both work
s = """Hi,
Python. I am new
here
"""
print(s)

# 4 
s = "welcome to my \"home\" "
print(s)
s = "welcome to my \npython "
print(s)
s1 = " simple \ space "
s2 = "backslash at end \\"
s3 = "\\n"
s4 = "\\t"

# 5  in raw string, escape seq are not processed
s1 = "c:\name\file.html"
s2 = r"c:\name\file.html"
print(s1)
print(s2)