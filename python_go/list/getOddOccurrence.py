# Given an array of positive integers. 
# All numbers occur an even number of times except one number which occurs an odd number of times. 
# Find the number in O(n) time & constant space.

# A Simple Solution is to run two nested loops. 
# The outer loop picks all elements one by one and the inner loop 
# counts the number of occurrences of the element picked by the outer loop. 
# The time complexity of this solution is O(n2).

# The Best Solution is to do bitwise XOR of all the elements. 
# XOR of all elements gives us odd occurring elements. 

# Here ^ is the XOR operators;
# Note :
# x^0 = x
# x^y=y^x (Commutative property holds)
# (x^y)^z = x^(y^z) (Distributive property holds)
# x^x=0


def getOddOccurrence(arr):

	# Initialize result
	res = 0
	
	# Traverse the array
	for element in arr:
		# XOR with the result
		res = res ^ element

	return res

# Test array
arr = [ 2, 3, 5, 4, 5, 2, 4, 3, 5, 2, 4, 4, 2]

print("%d" % getOddOccurrence(arr))