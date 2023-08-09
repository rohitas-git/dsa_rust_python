
def findSum(n) :
	return n * (n + 1) / 2


n = int(input("Enter n:"))
sum = findSum(n)
print(f"Sum is {sum}")
# The above program causes overflow, even if the result is not beyond the integer limit. 


# Better because We can avoid overflow up to some extent by dividing first.
def findSum(n) :
    if (n % 2 == 0) :
        return (n / 2) * (n + 1)
    else:
        return ((n + 1) / 2) * n

