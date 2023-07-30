from sys import argv

def main():
    N = int(argv[1])
    triangle(N)
    

def triangle(N: int):
    n=N
    total=2*n-1
    for i in range(1,total+1):
        if i<=N:
            print("* "*i)
        else:
            print("* "*(total-i))
        

main()