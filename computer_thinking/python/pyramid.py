from sys import argv

def main():
    input = int(argv[1])
    pyramid(input)
    reversed_pyramid(input)
    

def pyramid(n: int):
    
    for i in range(n):
        space = n -i - 1
        star = 2*i + 1
        print("- "*space + "* "*star + "- "*space)
        
def reversed_pyramid(n: int):
    for i in range(n-1,-1,-1):
        space = n -i - 1
        star = 2*i + 1
        print("- "*space + "* "*star + "- "*space)
    
    
main()