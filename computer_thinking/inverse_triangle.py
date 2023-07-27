from sys import argv

def main():
    input = int(argv[1])
    inverseLadder(input)
    inverseLadder2(input)


def inverseLadder(side:int):
    for i in range(side,0,-1):
        print("* "*(i))
    
def inverseLadder2(N:int):
    for i in range(N,0,-1):
        for j in range(1,i+1,1):
            print(j, end = ' ')
        print()
        
        
        
main()