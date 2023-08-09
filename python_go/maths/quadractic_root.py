import math

def quadraticRoots( a, b, c):
    roots = []
    root1=0
    root2=0
    #value of b^2-4ac
    temp=pow(b,2)-4*a*c
    #if b^2-4ac is less then zero then roots are imaginary
    if temp<0:
        roots.append(-1)
    else:
        # calculate root1 and root2 using fomula 
        # math.floor function returns greatest integer below ( -b + sqrt(temp) )
        # math.sqrt function returns square root of temp
        root1=math.floor((-1*b+math.sqrt(temp))/(2*a))  
        root2=math.floor((-1*b-math.sqrt(temp))/(2*a))
        #store both roots calculated in List
        # math.max function returns greater value between root1 and root2
        # math.min function returns smaller value between root1 and root2
        roots.append(max(root1,root2))
        roots.append(min(root1,root2))
    return roots