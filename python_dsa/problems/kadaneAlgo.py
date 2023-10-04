from os import *
from sys import *
from collections import *
import math

from sys import stdin,setrecursionlimit
setrecursionlimit(10**7)

def maxSubarraySum(arr, n) :

    n = len(arr)
    maxi = -math.inf
    summ = 0

    for i in range(n):
        summ+=arr[i]
        
        if summ > maxi:
            maxi = summ
        
        if summ < 0:
            summ = 0
    
    return maxi


#taking inpit using fast I/O
def takeInput() :
	
    n =  int(input())

    if(n == 0) :
        return list(), n

    arr = list(map(int, stdin.readline().strip().split(" ")))

    return arr, n

#main
arr, n = takeInput()
print(maxSubarraySum(arr, n))
