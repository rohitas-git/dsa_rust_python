from sys import argv

def main():
    N = int(argv[1])
    
    print_wings2(N)



# N = 5
# *        *
# **      **
# ***    ***
# ****  ****
# **********
# ****  ****
# ***    ***
# **      **
# *        *

def print_wings(N: int):
    
    for i in range(N-1):
        space = 2*N -2 -2*i
        star = 2 + 2*i
        star_half = int(star/2)

        print("*"*star_half, end="")
        print(" "*space, end="")
        print("*"*star_half, end="")     
        print()
    
    print("*"*2*N)
    
    for i in range(N-2,-1,-1):
        space = 2*N -2 -2*i
        star = 2 + 2*i
        star_half = int(star/2)

        print("*"*star_half, end="")
        print(" "*space, end="")
        print("*"*star_half, end="")     
        print()
    
def print_wings2(N: int):
    
    for i in range(2*N):
        stars= i
        space = 2*N -2 -2*i
        if i>N:
            stars= 2*N +1 - i
            space = -2*N -2 + 2*i
        print("*"*stars, end="")
        print(" "*space, end="")
        print("*"*stars, end="")     
        print()
main()