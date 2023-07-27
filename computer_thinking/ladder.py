from sys import argv

def main():
    ladder(int(argv[1]))


def ladder(side:int):
    for i in range(side):
        print("* "*(i+1))
        
main()