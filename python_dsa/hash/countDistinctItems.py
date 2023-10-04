# v1 faster than v2

def count_distinct_elements_v1(my_list):
    return len(set(my_list))
    

def count_distinct_elements_v2(my_list):
    res=1 
    for i in range(1,len(my_list)):
        if my_list[i] not in my_list[0:i]:
            res+=1
    return res


if __name__ == "__main__":
    l = [1,2,3,3,4,4,4,4,4]
    print(count_distinct_elements_v1(l))
    print(count_distinct_elements_v2(l))

# end main