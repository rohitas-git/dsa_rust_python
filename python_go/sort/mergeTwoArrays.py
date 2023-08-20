
def main():
    l1 = [10,15,20]
    l2 = [5,6,6,30]
    l3 = merge(l1,l2)
    

# time Theta(m+n), 
# space O(m+n)
def merge(l1, l2):
    n = len(l1)
    m = len(l2)
    
    l3 = []
    
    i,j, = 0, 0
    while i < n and j < m:
        
        if l1[i] < l2[j]:
            l3.append(l1[i])
            i+=1
        elif l1[i] > l2[j]: 
            l3.append(l2[j])
            j+=1
        

    while i<n:
        l3.append(l1[i]) 
        i+=1
    
    while j<m:
        l3.append(l2[j])
        j+=1
        
    print(l3)
    return l3




main()