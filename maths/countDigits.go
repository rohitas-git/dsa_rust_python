package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"
	"strconv"
	"strings"
)

func countDigits(x int) {
	var res int = 0
	t := x
	for x > 0 {
		x = x / 10
		res += 1
	}
	fmt.Printf("Number of digits in %d: %d \n", t, res)
}

// Optimal
func countDigits2(x float64){
	res:= math.Floor(math.Log10(x) + 1)
	fmt.Println(res)
}

func main() {
	fmt.Println("Enter the number:")

	reader := bufio.NewReader(os.Stdin)
	number, err := reader.ReadString('\n')
	if err == nil {
		if number, err := strconv.Atoi(strings.TrimSpace(number)); err == nil {
			countDigits2(float64(number))
		}
	} else {
		log.Fatal(err)
	}
}
