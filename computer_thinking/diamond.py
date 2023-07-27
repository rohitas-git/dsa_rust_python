# - - * - -
# - * * * -
# * * * * *
# - * * * -
# - - * - -

def sol1(length: int):
    
    half = int(length/2)
    
    # upper
    star = 1
    for i in range(0, half):
        empty = int((length - star)/2)
        # print(star, int(empty/2))
        row = ("-"*(empty) + "*"*(star) + "-"*(empty))
        print(row)
        star = star + 1*2
        
    # middle
    row = ("*"*(length))
    print(row)
    
    # lower
    star = length - 2
    for i in range(0, half):
        empty = int((length - star)/2)
        # print(star, int(empty/2))
        row = ("-"*(empty) + "*"*(star) + "-"*(empty))
        print(row)
        star = star - 1*2
      


# sol1(5)


def sol2(length: int):
    
    for i in range(0,length):
        if i < length/2 :
            star = 1 + (i)*2
            empty = int((length - star)/2)
            row = ("-"*(empty) + "*"*(star) + "-"*(empty))
            print(row)
        
        if i > length/2 :
            star = length - (i-int(length/2))*int((length-1)/2)
            empty = int((length - star)/2)
            row = ("-"*(empty) + "*"*(star) + "-"*(empty))
            print(row)
            
sol2(9)