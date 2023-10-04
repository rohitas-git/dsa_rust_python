
def main():
    l = [10,20,30,40]
    rotateByIteration(l)
    rotateBySlicing(l)
    rotateByPopAppend(l)

def rotateBySlicing(l):
    print("Before",l)
    l = l[1:] + l[0:1]
    print(l)
    
    
def rotateByPopAppend(l):
    print("Before",l)
    l.append(l.pop(0))
    print(l)
    
    
def rotateByIteration(l):
    print("Before",l)
    n = len(l)
    x = l[0]
    for i in range(1,n):
        l[i-1] = l[i]
    
    l[n-1] = x
    print(l)
    
main() 