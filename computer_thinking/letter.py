from sys import argv

def main():
    N = int(argv[1])
    
    letters(N)
    # printTriangle(N)    

def letters2(N: int):
    n=N
    for i in range(65, 65 + N):
        print(chr(i)*(i-65+1))
        

def letters(N: int):
    n=N
    for j in range(65, 65 + N):
        for i in range(65,j+1):
            print(chr(i),end="")
        print()
        
        
        
def printTriangle(N):
        spaces = N
        for i in range(65, 65 + N):
            for sp in range(1, spaces):
                print(' ', end = '')
            
            for j in range(65, i + 1):
                print(chr(j), end = '')
                
            for j in range(i - 1, 64, -1):
                print(chr(j), end = '')
            
            spaces -= 1
            print()
            
            
main()