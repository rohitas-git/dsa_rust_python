# Check if one string be obtained by rotation of other string

s1 = "abcd"
s2 = "cdab"

# Time - O(N)
def areRotations(s1,s2):
    if len(s1) == len(s2):
        return False

    temp = s1 + s1
    return temp.find(s2) != -1

print(areRotations(s1,s2))