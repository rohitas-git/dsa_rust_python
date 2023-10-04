# Comphrension 
# Shortcut syntax to create a list from another iterable

# Comprehensions in Python provide us with a short and concise way to construct new sequences 
# (such as lists, set, dictionary etc.) using sequences which have been already defined. 

# Python supports the following 4 types of comprehensions:
# List Comprehensions
# Dictionary Comprehensions
# Set Comprehensions
# Generator Comprehensions

# List Comprehensions:
# output_list = [output_exp for var in input_list if (var satisfies this condition)]

# Dictionary Comprehensions:
# output_dict = {key:value for (key, value) in iterable if (key, value satisfy this condition)}

# Set Comprehensions:
# pretty similar to list comprehensions.
# The only difference between them is that set comprehensions use curly brackets { }.

# Generator Comprehensions:
# Generator Comprehensions are very similar to list comprehensions. 
# One difference between them is that generator comprehensions use circular brackets 
# whereas list comprehensions use square brackets. 
# The major difference between them is that generators don’t allocate memory for the whole list. 
# Instead, they generate each value one by one which is why they are memory efficient.

# iterable - Range, list, dict, set, tuple

def main():
    
    l2 = [x for x in range(1,11) if x%2 !=0 ] # LIST
    # ex()
    dictn()
    

def generator():
    input_list = [1, 2, 3, 4, 4, 5, 6, 7, 7]

    output_gen = (var for var in input_list if var % 2 == 0)

    print("Output values using generator comprehensions:", end = ' ')

    for var in output_gen:
        print(var, end = ' ')



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