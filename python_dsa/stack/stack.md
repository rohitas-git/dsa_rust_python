
# Stack

It is a linear data structure that follows a particular order in which the operations are performed.
Order ->
LIFO (This strategy states that the element that is inserted last will come out first.)

In stack, a new element is added at one end and an element is removed from that end only. The insert and delete operations are often called push and pop.



Basic Operations on Stack
- push() to insert an element into the stack
- pop() to remove an element from the stack
- top() Returns the top element of the stack.
- isEmpty() returns true is stack is empty else false.
- size() returns the size of stack.
 
Time Complexity
Operations  	Complexity
push() 	    O(1)
pop()   	    O(1)
isEmpty() 	O(1)
size()	    O(1)

## Types of Stacks:
- Register Stack: This type of stack is also a memory element present in the memory unit and can handle a small amount of data only. The height of the register stack is always limited as the size of the register stack is very small compared to the memory.
- Memory Stack: This type of stack can handle a large amount of memory data. The height of the memory stack is flexible as it occupies a large amount of memory data. 

## Advantages of Stack:
- Stack helps in managing data that follows the LIFO technique.
- Stacks are be used for systematic Memory Management.
- It is used in many virtual machines like JVM.
- When a function is called, the local variables and other function parameters are stored in the stack and automatically destroyed once returned from the function. Hence, efficient function management.
- Stacks are more secure and reliable as they do not get corrupted easily.
- Stack allows control over memory allocation and deallocation.
- Stack cleans up the objects automatically.

## Disadvantages of Stack: 
- Stack memory is of limited size.
- The total of size of the stack must be defined before.
- If too many objects are created then it can lead to stack overflow.
- Random accessing is not possible in stack.
- If the stack falls outside the memory it can lead to abnormal termination.