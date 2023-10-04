# Function to return the count of non-repeated elements in the array.
def countNonRepeated(self, arr, n):
    dic = {}
    freq = 0

    # fill dic with K-V (<arr_item>,<item_freq>)
    for i in range(n):
        if arr[i] not in dic:
            freq = arr.count(arr[i])
            dic[arr[i]] = freq

    count = 0
    freq_values = dic.values()
    for val in freq_values:
        if val == 1:
            count += 1
    return count


# v1 faster than v2

def count_distinct_elements_v1(my_list):
    return len(set(my_list))


def count_distinct_elements_v2(my_list):
    res = 1
    for i in range(1, len(my_list)):
        if my_list[i] not in my_list[0:i]:
            res += 1
    return res


if __name__ == "__main__":
    l = [1, 2, 3, 3, 4, 4, 4, 4, 4]
    print(count_distinct_elements_v1(l))
    print(count_distinct_elements_v2(l))
