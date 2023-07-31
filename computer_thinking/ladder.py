from sys import argv


def main():
    input = int(argv[1])
    ladder4(input)


# star ladder
def ladder(side:int):
    for i in range(side):
        print("* "*(i+1))
      
        
# Repitive numeric ladder
def ladder2(side:int):
    for i in range(side):
        num= i+1
        print(f"{num} "*(i+1))


# Changing numeric ladder
def ladder3_0(side:int):
    for i in range(side):
        num=1
        for j in range(i+1):
            print(num, end=" ")
            num+=1
        print("")


# Changing numeric ladder
def ladder3_1(N: int):
    for i in range(1, 1 + N):
        for j in range(1, i + 1):
            print(j, end = ' ')
        print()


# binary ladder 
def ladder4(n:int):
    start = 1
    for i in range(n):
        here = start
        for j in range(i+1):
            print(here," ",end = "")
            here = (here+1) % 2
        print("")
        start = (start + 1) % 2


main()