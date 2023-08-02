from sys import argv

def main():
    n = int(argv[1])
    print(count_digits(n))


# Time Complexity: O(log(n)),
#   where ‘n’ is the given number.
    # We are iterating through all the digits of ‘n’ and there are log(n) such digits.
    # Hence, the time complexity is O(log(n)).

# Space Complexity: O(1).
    # We are not using any extra space.
    # Hence, the space complexity is O(1).
def count_digits(n:int) -> int:
    number = n
    count= 0
    
    while number > 0:
        digit = number % 10
        if digit !=0:
            if n % digit == 0: 
                count+=1 
        number = number // 10
    
    return count
    

main()