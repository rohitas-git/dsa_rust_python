
def main():
    l = [10,20,30,40]
    
    average(l)
    print(averageV2(l))
    
# time: O(n)
def average(l):
    sum=0
    for i in l:
        sum+=i
    average = sum/len(l)
    print(f"Average of List: {average}")
    return average


def averageV2(l):
    return sum(l)/len(l)


main()