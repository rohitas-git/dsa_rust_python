
#Function to insert elements of array in the hashTable avoiding collisions.
def separateChaining(hashSize, arr, sizeOfArray):

    hashtable = [[] for x in range(hashSize)]
    for item in arr:
        key = item % hashSize
        hashtable[key].append(item)
    
    return hashtable