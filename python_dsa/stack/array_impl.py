stackMax = 100000
stack = [-1]*stackMax 
top=-1

def push(data):
    ##Your code here
    global stack
    global top
    if top==100000:
        print("Stack Full")
    else:
        top+=1
        stack[top]=data

    
def pop():
    ##Your code here
    global stack
    global top
    if top==-1:
        print("Stack Empty")
    else:
        stack[top]==-1
        top-=1


def display():
    ##Your code here
    global stack
    global top
    if top==-1:
        print(-1)
    else:
        for i in range(0,top+1):
            print(stack[i],end=" ")
        print("")