
def main():
    n = 13
    votes = ['john', 'johnny', 'jackie', 'johnny', 'john', 'jackie', 'jamie', 'jamie' ,'john', 'johnny', 'jamie', 'johnny', 'john']
    print(winner(votes,n))

#Function to return the name of candidate that received maximum votes.
def winner(arr,n):
    d = {}
    max=0;
    for i in range(len(arr)):
        if arr[i] not in d:
            d[arr[i]] = 1  
        else:
            d[arr[i]] += 1
        if d[arr[i]] > max:
            max= d[arr[i]]
    
    winner = [person for person in arr if d[person]==max]
    winner.sort()
    print(max)
    return winner[0], max

main()