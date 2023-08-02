// Time: theta(logN)
// Aux space: O(1)
pub fn trailing_zeroe_in_factorial(n: u32){
    // Zeroes = Number of 5s in range(n)
	// count = [n/5] + [n/25] + [n/125] + ...

    let mut result = 0;
    let mut divider = 5;

    while divider <= n {
        result += n/divider;
        divider *= 5;
    }

    println!("Number of trailing 0s: {}", result );
}