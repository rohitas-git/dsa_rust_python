
// In classical programming, an n-bit integer is internally stored as 
// a binary number that consists of n bits.

// All possible n bit integers: 2^n
// Max possible value of n-bit integer: 2^n - 1
// 

// The bits are indexed from right to left.
// bit representation bk ···b2 b1 b0
// its value in base 10: bk*(2^k) +…+ b2*(2^2) + b1*(2^1) + b0*(2^0)

// A signed variable of n bits can contain any integer between -2^(n-1) and 2^(n-1) – 1
// An unsigned variable of n bits can contain any integer between 0 and 2^(n) −1.
// A signed number −x equals an unsigned number 2^n − x.

// Two’s complement is used, which means that the opposite number of a number is calculated 
// by first inverting all the bits in the number, and then increasing the number by one.

// The int number 43   = 00000000000000000000000000101011
// The int number −43  = 11111111111111111111111111010101
