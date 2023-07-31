package main

// var floor = math.Floor

// Time: theta(logN)
// Aux space: O(1)
func trailingZeroesInFactorial(n int) {
	// Zeroes = Number of 5s in range(n)
	// count = [n/5] + [n/25] + [n/125] + ...

	res := 0
	i := 5

	for i <= n {
		res = res + n/i
		i = i * 5
	}
	println(res)

}

func naiveSolution(n int) {
	fact := 1
	for i := 2; i < n+1; i++ {
		fact *= i
	}

	res := 0
	for fact%10 == 0 {
		res += 1
		fact = fact / 10
	}
	print(res)
}

func main() {

	// naiveSolution(20)
	trailingZeroesInFactorial(251)
}
