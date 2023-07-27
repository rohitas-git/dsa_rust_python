from sys import argv

def main():
    ladder(int(argv[1]))


def ladder(side:int):
    for i in range(side):
        num= i+1
        print(f"{num} "*(i+1))
        
main()