from sys import argv

def main():
    square(int(argv[1]))


def square(side:int):
    for i in range(side):
        print("* "*(side))
        
main()