# Slicing (List, tuple, string )

#   List[start: stop: step]
#   Tuple[start: stop: step]
#   String[start: stop: step]
#
#  stop is not included

def main():
    # listSlicing()
    diffSlicing()



def diffSlicing():
    l1 = [10,20,30]
    l2 = l1[:]
    
    t1= (10,20,30)
    t2 = t1[:]
    
    s1="geeks"
    s2=s1[:]
    
    print(l1 is l2) # different lists 
    print(t1 is t2) # same object returned 
    print(s1 is s2) # same object returned 



    

def listSlicing():
    l = [x for x in range(0,10)]
    
    slicedEven = l[2:100:2]
    slicedOdd = l[1:100:2]
    
    print("Even", slicedEven)
    print("Odd", slicedOdd)
    print(l[:5])
    print(l[5:])
    print(l[10: :-1])
    print(l[-1: -11 :-1])
    print(l[::-1]) # shortcut to get reverse of list
    print(l[:])
    

main()