package main

import (
	"fmt"
	"math"
	"os"
	"strconv"
)


// Time: theta(logN) where N is integer
// Aux space: theta(1)
func armstrongNumber(n int) {
	num := n
	original := n
	count := 0
	// find number of digits in number is base-10
	
	for n > 0 {
		n = n / 10
		count += 1
	}

	// calculate sum of pow(digit, no.of digits) for all digits
	sum := 0
	for num > 0{
		digit := num % 10
		sum += int(math.Pow(float64(digit), float64(count)))
		num = num /10
	}

	// check if sum == n
	if sum == original {
		fmt.Printf("Yes, it's an Armstrong Number\n")
	} else {
		fmt.Printf("No, it is not an Armstrong Number\n")
	}

}

func main() {
	args := os.Args[1:]
	if num, err := strconv.Atoi(args[0]); err == nil {
		armstrongNumber(num)
	}
}
