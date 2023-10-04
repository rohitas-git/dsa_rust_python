import math

def main():
    print(exactly3Divisors(6))
    

# 6 - rt(6) = 2.44 (find prime numbers in range(2.44) - 2 )
# rt(2) = 1.414 (divisor of prime is less 1.414 == 1 int)

# 68 - rt(68) -- 8.24 -- primes in range(8.24) -- 2,3,5,7

def exactly3Divisors(N):
        
        # the req number are square numbers in range(N)
        # if a number is perfect sq, then int(sqrt(i))**2 = i
        # but 64 is perfect sq but has more than 3 divisors
        
        l = int(math.sqrt(N))
        count = 0
        if l < 2:
            return 0
        for i in range(2,l+1):
            if i == 2 or i == 3:
                count+=1
            if i == 2 or i == 3:
                continue
            
            k = 5
            while k*k <= i:
                if i%k==0 or i% (k+2) == 0:
                    continue
                k+=6
            count+=1
            
        
        return count
    
main()