from sys import argv

def main():
    ladder(int(argv[1]))


def ladder(side:int):
    for i in range(side):
        num=1
        for j in range(i+1):
            print(num, end=" ")
            num+=1
        print("")

def ladder2(N: int):
    for i in range(1, 1 + N):
        for j in range(1, i + 1):
            print(j, end = ' ')
        print()
        
main()

