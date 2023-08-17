# Sorting in Python

# .sort()
# - works only for lists
# - Sorts in-palce

# sorted( )
# - works for any iterable container
# - Returns a list of sorted items

#! Both use TimSort and both are stable

# TimSort - Hybrid algorithm that uses mergeSort and InsertionSort internally.
# They used in a way that it makes it a optimized algorithm
# Worst case Time - O(nlogn)


# --------------- sort() ---------------------

l1 = [5,10,15,20]
l1.sort()
print(l1)

l2 = [1,3,5,10,6,2]
l3 = sorted(l2, reverse=True)
print(l3)

l4 = ["gfg", "game","archery"]
l4.sort()
print(l4)


def myFun(s):
    return len(s)

l4.sort(key= myFun)
print(l4)

# Note that key takes a fn which returns value according to which list is orted

# -----------------------Sorting user defined objects -----------------
# 1. Using a separate method (using key)
# 2. using __lt__ method 

# ----------1.----------------

class Point:

    def __init__(self, x, y):
        self.x = x
        self.y = y


def myFun(p):
    return p.x


l = [Point(1, 15), Point(10, 5), Point(3, 8)]
l.sort(key=myFun)

for i in l:
    print(i.x, i.y)

#--------------2-----------------

class Point:

    def __init__(self, x, y):
        self.x = x
        self.y = y

    def __lt__(self, other):
        return self.x < other.x


l = [Point(1, 15), Point(10, 5), Point(5, 8)]
l.sort()

for i in l:
    print(i.x, i.y)
    
# ----------- Comparison-------------------

# 2.->more useful when sorting according to natural order
# 1.->more useful when sorting using a custom order rather than natural order


# Sorted( )
# - works for any iterable container
# - Returns a list of sorted items
# - uses timSort and is stable

# for tuples, set, dict, list

# for user defined objects 
# - using __lt__ method
# - using key argument in sorted(key= ...)


#  ------------- Sorted( )-------------

l = [10, 20, 14]
ls = sorted(l)

print(l)
print(ls)

l = [10, -15, -2, 1]
ls = sorted(l, key=abs, reverse=True)
print(ls)



t = (10,12,5,1)
print(sorted(t))

s = {'gfg','courses','python'}
print(sorted(s))

st = 'gfg'
print(sorted(st))

d = {10:'gfg',15:'ide',5:'courses'}
print(sorted(d))

l = [(10,15),(1,8),(2,3)]
print(sorted(l))