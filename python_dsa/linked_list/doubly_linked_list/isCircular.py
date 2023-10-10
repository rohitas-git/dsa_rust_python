
# check if a doubly-linked list is circular or not
def isCircular(head):
    if head.prev != None:
        return 1
    else:
        return 0
