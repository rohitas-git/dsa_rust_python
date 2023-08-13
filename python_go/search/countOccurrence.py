from firstOccurrence import getFirstOccurrenceIterative
from lastOccurrence import getLastOccurrenceIterative

def getOccurrencesIterative(arr,x):   
    first = getFirstOccurrenceIterative(arr,x)
    if first == -1:
        return 0
    else:
        return getLastOccurrenceIterative(arr,x) - first + 1
                

arr = [10,15,20,20,20,30,40,40,40,40]
print(getOccurrencesIterative(arr,20))
print(getOccurrencesIterative(arr,40))


# Similar code for recursive fn