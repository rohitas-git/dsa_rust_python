# For a number, check if its palindrome

def isPalin(N):
        return check(N,N,0)
    
def check(number,original,newNumber):
    if original==0:
        if newNumber==number:
            return 1
        else:
            return 0
    
    newNumber=newNumber*10+(original%10)
    original=original//10
    
    return check(number,original,newNumber)