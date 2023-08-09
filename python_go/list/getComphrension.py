# Comphrension 
# Shortcut syntax to create a list from another iterable

# iterable - Range, list, dict, set, tuple

def main():
    
    
    l2 = [x for x in range(1,11) if x%2 !=0 ] # LIST
    
    
    # ex()
    dictn()
    

def dictn():
    l = [x for x in range(1,11) ]
    d = {x:x**3 for x in l}
    print(d)
    
    d = {x:f"ID{x}" for x in l}
    
    l2 = [101,102,103]
    l3 = ["gfg", "ide","course"]
    
    # one way
    d = {l2[i]:l3[i] for i in range(len(l2))}
    # Better way
    d = dict(zip(l2,l3))
    print(d)
    
    # inverting dictionary
    d2 = {v:k for (k,v) in d.items()}
    print(d2)
    


def set():
    l = [10,20,10,20,3,3,7,9]
    l1 = {x for x in l if x%2 ==0 } # SET - Contains distinct items
    print(l1)
       

def ex():
    s1="geeksforgeeks"
    l1 = [x for x in s1 if x in "aeiou"]
    print(l1)
    
    s1=["geeks", "down","gown","clown"]
    l1 = [x.upper() for x in s1 if x.startswith("g")]
    print(l1)
    
    
main()