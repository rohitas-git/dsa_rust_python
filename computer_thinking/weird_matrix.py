from sys import argv

def main():
    N = int(argv[1])
    
    # empty_sq(N)
    print_sq2(N)



def empty_sq(N: int):
    n=N
    for i in range(N):
        for j in range(N):
            if i==0 or j==0 or i==n-1 or j==n-1:
                print("*",end="")
            else:
                print(" ",end="")
        print()

# N = 4
# 4 4 4 4 4 4 4
# 4 3 3 3 3 3 4
# 4 3 2 2 2 3 4
# 4 3 2 1 2 3 4
# 4 3 2 2 2 3 4
# 4 3 3 3 3 3 4
# 4 4 4 4 4 4 4
def print_sq(N: int):
    total = 2*N-1
    for i in range(total):
        for j in range(total):
            top = i
            left = j
            right = total-j-1
            bottom=total-i-1
            
            this = N - min(top,left,right,bottom)
            # print(min(top,left,right,bottom))
            print(this,"" ,end="")
        print()
        
        
def print_sq2(n: int):
    for i in range(1, 2*n):
        for j in range(1, 2*n):
            row, col = i, j
            if row > n:
                row = 2 * n - row
            if col > n:
                col = 2 * n - col
            print(n - min(row, col) + 1, end = ' ')
        print()
main()