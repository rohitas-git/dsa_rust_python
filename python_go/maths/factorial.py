
# Recursive approach
# Time Complexity: O(n)
# Auxiliary Space: O(n)
def factorialxR(n):
	
	# single line to find factorial
	return 1 if (n==1 or n==0) else n * factorialxR(n - 1);


# Iterative approach
# Time Complexity: O(n)
# Auxiliary Space: O(1)
def factorialxI(n):
	if n < 0:
		return 0
	elif n == 0 or n == 1:
		return 1
	else:
		fact = 1
		while(n > 1):
			fact *= n
			n -= 1
		return fact

