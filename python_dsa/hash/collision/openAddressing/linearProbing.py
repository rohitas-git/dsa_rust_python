from enum import Enum
 
class Status(Enum):
    Empty = 1
    Filled = 2
    Deleted = 3
    
# Function to fill the array elements into a hash-table
# using linear probing to handle collisions
def linearProbing(hashSize, arr, sizeOfArray):
    
    hashTable= [Status.Empty]*hashSize
    count=0

    for i  in range(sizeOfArray):
        if count == hashSize:
            break
        
        item = arr[i]
        x = item
        index = item % hashSize
        alreadyInserted = False
        
        while hashTable[index] != Status.Empty:
            # if already inserted, break
            if hashTable[index] == item:
                alreadyInserted = True
                break 
            
            # Move one step forward (Linear Probing)
            x = x + 1
            index = x % hashSize
            
        if alreadyInserted:
            continue
        else:
            hashTable[index] = item 
            count+=1
    
    return hashTable


def main():
    arr = [44,45,79,55,91,18,63]
    arr = [12,18,13,2,3,23,5,15]
    arr = []
    n = len(arr)
    print(linearProbing(10,arr,n))
    
main()