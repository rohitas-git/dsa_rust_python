from sys import argv

def main():
    n = int(argv[1])
    reverse_bits(n)
    print(reverseBits(n))



# Time complexity:  
# Space complexity: 
def reverse_bits(n: int):
    
    reversed = int_to_binary(n)
    reversed = reversed[::-1]
    reversed = int(reversed,2)
    
    print(reversed)
    # print(binary)


def int_to_binary(integer):
    binary_string = ''
    while(integer > 0):
        digit = integer % 2
        binary_string += str(digit)
        integer = integer // 2
    binary_string = binary_string[::-1]
    return binary_string


"""
    Time Complexity: O(1)
    Space Complexity: O(1)
"""
def reverseBits(n):
    # 'bits' array to store the bits representation of 'n'.
    bits = [0]*32

    # Preparing the bits array.
    for i in range(32):
        # If the ith bit is set.
        if (n & (1 << i)) > 0:
            bits[i] = 1

    for i in range(16):
        # Swapping the left and rightmost bits.
        bits[i], bits[31-i] = bits[31-i], bits[i]

    # 'ans' will store the value for the reversed bits representation.
    ans = 0

    for i in range(32):
        # If the i'th bit is set.
        if bits[i] == 1:
            ans += (1 << i)

    return ans


main()