
# Deque (Doubly Ended Queue)

Deque is preferred over a list in the cases where we need quicker append and pop operations from both the ends of the container, as deque provides an O(1) time complexity for append and pop operations as compared to a list that provides O(n) time complexity.

## Types of Restricted Deque Input
Input Restricted Deque:  Input is limited at one end while deletion is permitted at both ends.
Output Restricted Deque: output is limited at one end but insertion is permitted at both ends.

## Operations on Deque :

insertFront() : Adds an item at the front of Deque.
insertRear()  : Adds an item at the rear of Deque.
deleteFront() : Deletes an item from front of Deque.
deleteRear()  : Deletes an item from rear of Deque.