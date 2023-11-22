

# Bitwise Operators
* The & (bitwise AND) in C or C++ takes two numbers as operands and does AND on every bit of two numbers. The result of AND is 1 only if both bits are 1.  
* The | (bitwise OR) in C or C++ takes two numbers as operands and does OR on every bit of two numbers. The result of OR is 1 if any of the two bits is 1. 
* The ^ (bitwise XOR) in C or C++ takes two numbers as operands and does XOR on every bit of two numbers. The result of XOR is 1 if the two bits are different. 
* The << (left shift) in C or C++ takes two numbers, the left shifts the bits of the first operand, and the second operand decides the number of places to shift. 
* The >> (right shift) in C or C++ takes two numbers, right shifts the bits of the first operand, and the second operand decides the number of places to shift. 
* The ~ (bitwise NOT) in C or C++ takes one number and inverts all bits of it.

## Interesting Facts About Bitwise Operators
1. The left-shift and right-shift operators should not be used for negative numbers.
2. Interestingly!! The bitwise OR of two numbers is just the sum of those two numbers if there is no carry involved, otherwise, you just add their bitwise AND.
3. The bitwise XOR operator is the most useful operator from a technical interview perspective.
4. The Bitwise operators should not be used in place of logical operators.