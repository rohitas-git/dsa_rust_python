# DICTIONARY in python
# - Collection of key-value pairs
# - Unordered
# - All keys must be distinct
# - Values may be repeated
# - Uses hashing internally 


#---------------------Creating Dictionary: Storing K-V pairs---------------------------
d = {10: True, 101: "abc", "name":"abc"}

d ={}
d["name"] = "Karan"
d["age"] = 20
d[100] = 10
print(d)

#--------------------Access values using keys------------
print(d["name"])
print(d.get("name"))

# if key doesn't exist in dictionary => output: None
print(d.get(125))

if 125 in d:
    # raises 'Key Error' if key not present
    print(d[125])
else:
    print('NA')
    
# shortcut for above code block: prints "NA" if key not present
print(d.get(125, "NA"))

#------------------------Removal------------------
d={1:1,2:2,3:3,4:4}

d[1] = 5
print(len(d))
print(d) 

# remove key-value pair and returns the deleted value
print('returning and removing 105', d.pop(105))
print(d) 

# delete key-value pair
del d[3]
print(d)

d[5]=10
# delete last inserted k-v pair and returns tuple of deleted (k,v) pair
print('returning and removing last inserted', d.popitem())

