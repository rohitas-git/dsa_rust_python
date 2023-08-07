
def main():
    addingElementsV1()


# List Methods
# Function	Description
# Append()	Add an element to the end of the list
# Extend()	Add all elements of a list to another list
# Insert()	Insert an item at the defined index
# Remove()	Removes an item from the list
# Clear()	Removes all items from the list
# Index()	Returns the index of the first matched item
# Count()	Returns the count of the number of items passed as an argument
# Sort()	Sort items in a list in ascending order
# Reverse()	Reverse the order of items in the list
# copy()	Returns a copy of the list

# Built-in functions with List
# Function	Description
# reduce()	apply a particular function passed in its argument to all of the list elements stores the intermediate result and only returns the final summation value
# sum()	Sums up the numbers in the list
# ord()	Returns an integer representing the Unicode code point of the given Unicode character
# cmp()	This function returns 1 if the first list is “greater” than the second list
# max()	return maximum element of a given list
# min()	return minimum element of a given list
# all()	Returns true if all element is true or if the list is empty
# any()	return true if any element of the list is true. if the list is empty, return false
# len()	Returns length of the list or size of the list
# enumerate()	Returns enumerate object of the list
# accumulate()	apply a particular function passed in its argument to all of the list elements returns a list containing the intermediate results
# filter()	tests if each element of a list is true or not
# map()	returns a list of the results after applying the given function to each item of a given iterable
# lambda()	This function can have any number of arguments but only one expression, which is evaluated and returned.

def inputStringList():
    # Python program to take space
    # separated input as a string
    # split and store it to a list
    # and print the string list

    # input the list as string
    string = input("Enter elements (Space-Separated): ")

    # split the strings and store it to a list
    lst = string.split()  
    print('The list is:', lst)   # printing the list
    

def inputIntegerList():
    
    # input size of the list
    n = int(input("Enter the size of list : "))
    # store integrs in a list using map,
    # split and strip functions
    lst = list(map(int, input("Enter the integer\
    elements:").strip().split()))[:n]

    # printing the list
    print('The list is:', lst)   
    

def addingElementsV1():
    # Python program to demonstrate
    # Addition of elements in a List

    # Creating a List
    List = []
    print("Initial blank List: ")
    print(List)

    # Addition of Elements
    # in the List
    List.append(1)
    List.append(2)
    List.append(4)
    print("\nList after Addition of Three elements: ")
    print(List)

    # Adding elements to the List
    # using Iterator
    for i in range(1, 4):
        List.append(i)
    print("\nList after Addition of elements from 1-3: ")
    print(List)

    # Adding Tuples to the List
    List.append((5, 6))
    print("\nList after Addition of a Tuple: ")
    print(List)

    # Addition of List to a List
    List2 = ['For', 'Geeks']
    List.append(List2)
    print("\nList after Addition of a List: ")
    print(List)
    
    # Complexities for Adding elements in a Lists(append() method):
    # Time Complexity: O(1) 
    # Space Complexity: O(1)
    
def addingElementsV2():

    # Python program to demonstrate 
    # Addition of elements in a List
    
    # Creating a List
    List = [1,2,3,4]
    print("Initial List: ")
    print(List)

    # Addition of Element at 
    # specific Position
    # (using Insert Method)
    List.insert(3, 12)
    List.insert(0, 'Geeks')
    print("\nList after performing Insert Operation: ")
    print(List)
    
    # Complexities for Adding elements in a Lists(insert() method):
    # Time Complexity: O(n) 
    # Space Complexity: O(1)
    

def addingElementsV3():
    # Python program to demonstrate
    # Addition of elements in a List

    # Creating a List
    List = [1, 2, 3, 4]
    print("Initial List: ")
    print(List)

    # Addition of multiple elements
    # to the List at the end
    # (using Extend Method)
    List.extend([8, 'Geeks', 'Always'])
    print("\nList after performing Extend Operation: ")
    print(List)
    

main()