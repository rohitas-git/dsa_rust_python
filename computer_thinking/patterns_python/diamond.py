# - - * - -
# - * * * -
# * * * * *
# - * * * -
# - - * - -

from sys import argv

def main():
    input = int(argv[1])
    # diamond(input)
    print("")
    # half_diamond(input)
    
    numeric_cleave2(input)

def diamond(n:int):
    pyramid(n)
    reversed_pyramid(n)
        

def pyramid(n: int):
    
    for i in range(n):
        space = n -i - 1
        star = 2*i + 1
        print("  "*space + "* "*star + "  "*space)
        
        
def reversed_pyramid(n: int):
    for i in range(n-1,-1,-1):
        space = n -i - 1
        star = 2*i + 1
        print("  "*space + "* "*star + "  "*space)
        
        
def half_diamond(n:int):
    for i in range(n):
        space = n -i - 1
        star = i + 1
        print("* "*star + " "*space)
    
    for i in range(n-2,-1,-1):
        space = n -i - 1
        star = i + 1
        print("* "*star + " "*space)
        
        
def numeric_cleave(n:int):
    for i in range(n-1,-1,-1):
        num = n -i - 1
        space = 2*i 
        
        for k in range(num+1):
            print(k+1, end="")
            
        print(" "*(space), end="")
        
        for k in range(num+1,0,-1):
            print(k, end="")
        print()
    

def numeric_cleave2(n:int):
    for i in range(n-1,-1,-1):
        num = n -i - 1
        space = i*4
        
        for k in range(num+1):
            print(k+1,"", end="")
            
        # print(" "*(4*n-2 - 2*(2*num+1)), end="")
        print(" "*(space), end="")
        
        for k in range(num+1,0,-1):
            print(k,"", end="")
        print()
    
main()