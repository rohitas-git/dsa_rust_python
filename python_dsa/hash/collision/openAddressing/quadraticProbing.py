
# See Quadratic / Mid-square Probing
# Quadratic probing is a collision handling technique in hashing. 
# Quadratic probing says that whenever a collision occurs, search for i^2 position

from enum import Enum
 
class Status(Enum):
    Empty = 1
    Filled = 2
    Deleted = 3
    
# Function to fill the array elements into a hash-table
# using linear probing to handle collisions
def quadraticProbing(hashSize, arr, sizeOfArray):
    
    hashTable= [Status.Empty]*hashSize
    count=0

    for i  in range(sizeOfArray):
        if count == hashSize:
            break
        
        item = arr[i]
        x = item
        index = item % hashSize
        alreadyInserted = False
        iterationCount = 0
    
        while hashTable[index] != Status.Empty:
            # if already inserted, break
            if hashTable[index] == item:
                alreadyInserted = True
                break 
        
            iterationCount+=1
            
            # Move i*i step forward (Quadratic Probing)
            x = item + iterationCount*iterationCount
            index = x % hashSize
            # print('iter: ', iterationCount)
            # print('x: ',x)
            # print('index: ', index)
            # print('--------')
        
        if alreadyInserted:
            continue
        else:
            # print("Put ", item, "at ", index)
            # print('--------------------------------------------------')
            hashTable[index] = item 
            count+=1
    
    return hashTable


if __name__ == "__main__":
    hashSize = 11 
    N = 4
    arr = [21,10,32, 43]
    print(quadraticProbing(hashSize, arr, N))
# end main        
