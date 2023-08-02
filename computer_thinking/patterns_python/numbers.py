from sys import argv
import time

def main():
    input = int(argv[1])

    # st = time.time()
    natural_numbers(input)    
    # et = time.time()    
    # elapsed_time = et - st
    # print('Execution time:', elapsed_time, 'seconds')
    
    # st = time.time()
    natural_numbersV2(input)    
    et = time.time()    
    # elapsed_time = et - st
    # print('Execution time2:', elapsed_time, 'seconds')
    
    
def natural_numbers(n:int):
    start = 1
    for row in range(1,n+1,1):
        
        for j in range(row):
            print(start,"",end="")
            start+=1
        print()
        

def natural_numbersV2(n:int):
    total = int(n*(n+1)/2)
    nums = list( range(1,total+1) )
    last = 0
    for i in range(n):
        print(nums[last:last+1+i])
        last = last + 1 + i
        

    
main()