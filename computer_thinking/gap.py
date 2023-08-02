from sys import argv


def main():
    N = int(argv[1])    
    
    print_gap(N)
    

# N = 5
# **********
# ****  ****
# ***    ***
# **      **
# *        *
# *        *
# **      **
# ***    ***
# ****  ****
# **********
def print_gap(N: int):
    upper(N)
    lower(N)

def upper(N: int):
    
    for i in range(N):
        star = N*2 - 2*(i)
        star_half = int(star / 2)
        space = 2*(i)    
        
        print("*"*star_half, end="")
        print(" "*space, end="")
        print("*"*star_half, end="")     
        print()    

def lower(N: int):
    
    for i in range(N-1,-1,-1):
        star = N*2 - 2*(i)
        star_half = int(star / 2)
        space = 2*(i)    
        
        print("*"*star_half, end="")
        print(" "*space, end="")
        print("*"*star_half, end="")
        print()  
    
main()