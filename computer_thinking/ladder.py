from sys import argv


def main():
    input = int(argv[1])
    ladder(input)


def ladder(side:int):
    for i in range(side):
        print("* "*(i+1))
        

def ladder2(side:int):
    for i in range(side):
        num= i+1
        print(f"{num} "*(i+1))


def ladder3_0(side:int):
    for i in range(side):
        num=1
        for j in range(i+1):
            print(num, end=" ")
            num+=1
        print("")


def ladder3_1(N: int):
    for i in range(1, 1 + N):
        for j in range(1, i + 1):
            print(j, end = ' ')
        print()

main()