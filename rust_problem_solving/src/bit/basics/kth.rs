
// Finding a bit: Find the Kth bit in binary representation of N.
// Setting a bit: If Kth bit is 0, then set it to 1. Otherwise, leave it unchanged.
// Clearing a bit: If Kth bit is 1, then clear it to 0. Otherwise, leave it unchanged.
// Toggling a bit: If Kth bit is 1, then change it to 0 and vice-versa.
// Modifying a bit: Replace the Kth bit with a given bit.

// Setting  > |
// Clearing &
// Toggle  ^
// Finding &
// Modify | 

// Finding a bit:
//  (N >> K) & 1

// Setting a bit:
//  N = N | (1 << K)

// Clearing a bit:
//  N = N & ~(1 << K)
    
// Toggle a bit:
//  N = N ^ (1 << K)
    
// Modify a bit:
//  N = N | (P << K)
    