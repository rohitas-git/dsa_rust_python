from sys import argv


def main():
    n = int(argv[1])
    countDigits(n)
    
def countDigits(x:int):
    res = 0
    tmp = x
    while tmp > 0:
        tmp = tmp // 10
        res+=1
	
    print("Digits in", x, ":",res)


# Optimal
def countDigitsv2(n: int):
    k = 


main()