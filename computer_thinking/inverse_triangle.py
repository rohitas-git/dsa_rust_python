from sys import argv

def main():
    inverseLadder(int(argv[1]))


def inverseLadder(side:int):
    for i in range(side,0,-1):
        print("* "*(i))
    
def inverseLadder2(side:int):
    for i in range(side,0,-1):
        print("* "*(i))
        
main()