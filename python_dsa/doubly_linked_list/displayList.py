

def displayList(head):
    data=[]
    curr=head
    while True:
        data.append(curr.data)
        curr=curr.next
        if curr==head:
            return data 