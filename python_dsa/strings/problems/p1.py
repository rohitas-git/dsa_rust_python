
"""
Given a string s representing a password, you need to check if the string is valid or not. 
A valid string has the following properties:

String must have the length greater than or equal to 10.
String must contain at least 1 numeric character.
String must contain at least 1 uppercase character.
String must contain at least 1 lowercase character.
String must contain at least 1 special character from @#$-*.
"""


def validate(s):
    # your code here
    present = {}
    specials = "@#$-*"

    if len(s) < 10:
        return 0

    hasNumeric, hasUpper, hasLower, hasSpecial = False, False, False, False
    for char in s:
        ichar = ord(char)

        if ichar >= 49 and ichar <= 57:
            hasNumeric = True

        if ichar >= 65 and ichar <= 90:
            hasUpper = True

        if ichar >= 97 and ichar <= 122:
            hasLower = True

        if char in specials:
            hasSpecial = True

    if hasNumeric and hasUpper and hasLower and hasSpecial:
        return 1
    else:
        return 0
