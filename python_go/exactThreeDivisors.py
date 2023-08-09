import math

def main():
    n = int(input("Enter number: "))
    print(exactly3Divisors(n))

def isPrime(N):
        if (N==1):
            return False
        for i in range(2,1+int(math.sqrt(N))):
            if N%i==0:
                return False
        return True
        
# function to check the numbers with exactly 3 divisors
def exactly3Divisors(N):
    N = int(math.sqrt(N))
    #Initializing counter to zero
    counter=0 
    #Running a loop from 1 to sqrt(N)
    for i in range(1,N+1): 
    
        # A number N only has 3 divisors if it is a  
        # perfect square and its square  root is a prime number, 
        # and we know the number of perfect squares less than N which 
        # is sqrt(N), so just checking if i is prime or not
        if(isPrime(i)): 
            counter+=1
    return counter

main()