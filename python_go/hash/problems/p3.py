
def main():
    s = "hello"
    print(nonrepeatingCharacter(s))

# Function to find the first non-repeating character in a string.
def nonrepeatingCharacter(s):
    chars = [char for char in s]
    print(chars)
    
    d = {}
    for i in range(len(chars)):
        if chars[i] not in d:
            d[chars[i]] = 1
        else:
            d[chars[i]] += 1
        
    non_repeated_elements = [item for item in chars if d[item]== 1]
    if len(non_repeated_elements) == 0:
        return '$'
    else:
        return non_repeated_elements[0]
    
main()