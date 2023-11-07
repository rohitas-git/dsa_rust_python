
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
        if x in ('(', '[', '{'):
            stack.append(x)
        else:
            if not stack:
                return False
            elif isMatching(stack[-1], x) == False:
                return False
            else:
                stack.pop()
    if stack:
        return False
    else:
        return True


def isMatching(a, b):
    if (a == '(' and b == ')') or \
        (a == '{' and b == '}') or \
            (a == '[' and b == ']'):
        return True
    else:
        return False

# --------------------------------sol2----------------------------


def check(expression):

    open_tup = tuple('({[')
    close_tup = tuple(')}]')
    map = dict(zip(open_tup, close_tup))
    queue = []

    for i in expression:
        if i in open_tup:
            queue.append(map[i])
        elif i in close_tup:
            if not queue or i != queue.pop():
                return "Unbalanced"
    if not queue:
        return "Balanced"
    else:
        return "Unbalanced"


# Driver code
string = "{[]{()}}"
print(string, "-", check(string))

string = "((()"
print(string, "-", check(string))


# ------------------------sol3-----------------------------------
# Python3 code to Check for
# balanced parentheses in an expression
def check(my_string):
    brackets = ['()', '{}', '[]']
    while any(x in my_string for x in brackets):
        for br in brackets:
            my_string = my_string.replace(br, '')
    return not my_string


# Driver code
string = "{[]{()}}"
print(string, "-", "Balanced"
      if check(string) else "Unbalanced")
