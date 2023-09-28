
# ! Separate chaining technique in hashing allows us to 
# use a linked list at each hash slot to handle the problem of collisions. 
# That is, every slot of the hash table is a linked list, 
# so whenever a collision occurs, the element can be appened as a node to the linked list at the slot.

from enum import Enum
 
# Each node will contain a key-value pair, as well as a pointer to the next node in the list.
class Node:
    def __init__(self, key, data):
        self.key = key
        self.data = data
        self.next = None


class LinkedList:
    def __init__(self,head=None):
        self.head = head    
    
    def find(self, target):
        if self.head is None:
            return False
        
        current_node = self.head 
        while(current_node.next):
            if current_node.data == target:
                return True 
            else:
                current_node = current_node.next
        return False

    def insertAtBegin(self,key, data):
        new_node = Node(key, data)
        if self.head is None:
            self.head = new_node
            return 
        else:
            new_node.next = self.head
            self.head = new_node
        
            
    def insertAtEnd(self, key, data):
        new_node = Node(key, data)
        
        # if list is empty
        if self.head is None:
            self.head = new_node
            return 
        
        # move to end of list
        current_node = self.head 
        while(current_node.next):
            current_node = current_node.next
        
        current_node.next = new_node


class HashTable:
    def __init__(self, capacity):
        self.capacity = capacity
        self.size = 0
        self.table = [LinkedList() for x in range(capacity)] 


def separateChaining(hashSize, arr, N):
    hashTable= HashTable(hashSize)
    
    for i  in range(N):
        if hashTable.size == hashTable.capacity:
            break
        
        item = arr[i]
        index = item % hashSize
        alreadyExist = False
        curr_list = hashTable.table[index]
        
        if curr_list != None :   
            # if already exist, break
            if curr_list.find(item):
                alreadyExist = True
                break 
            
            # append it to the list
            curr_list.insertAtBegin(item, item)
            return 
            
        if alreadyExist:
            continue
        else:
            curr_list.head = Node(item,item)
            hashTable.size +=1
    print(hashTable.table[9].head)
    return hashTable





if __name__ == "__main__":
    hashSize = 11 
    N = 4
    arr = [21,10,32, 43]
    hash_table = separateChaining(hashSize, arr, N)
    
    for i, linked_list in enumerate(hash_table.table):
        current_node = linked_list.head
        print(f"LinkedList {i}: ", end="")
        while current_node:
            print(f"({current_node.key}: {current_node.data})", end=" -> ")
            current_node = current_node.next
        print("None")
        # print(HashTable(4).table)
# end main