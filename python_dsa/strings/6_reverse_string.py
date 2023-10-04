
#--------Reverse fn inbuilt-------
s = "abcdefg"
print()

# --------String slicing--------
# s[x:y:z] gives substring from x to y in steps of z
s = "geek"
print(s[::-1])


# ----- my own fn--------
def my_reverse():
    s = input("Enter a string: ")
    rev =""
    for i in s:
        rev = i + rev
    print(rev)
    
# my_reverse()