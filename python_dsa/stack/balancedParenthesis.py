
# Check  for Balanced Parenthesis

# possible chars: {}, [], ()

# A string is said to have balanced parenthesis 
# if the last open parenthesis is closed first 

# not balanced: "([)]"
# balanced: {}([()])


# time - O(n)
# aux space - O(n)
def isBalanced(expr):
    stack = []
    
    for x in expr:
        if x in ('(', '[','{'):
            stack.append(x)
        else:
            if not stack:
                return False 
            elif isMatching(stack[-1], x) ==  False:
                return False
            else:
                stack.pop()
    if stack:
        return False
    else:
        return True
    

def isMatching(a,b):
    if (a=='(' and b==')') or \
        (a=='{' and b=='}') or \
        (a=='[' and b==']'):
            return True
    else:
        return False