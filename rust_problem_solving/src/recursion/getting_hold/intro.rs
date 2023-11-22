/* ---------------------------- What is recursion ?--------------------------- */

// In computer science, recursion occurs when a function calls itself within its declaration.

// We use recursion to solve a large problem by breaking it down into smaller instances of the same problem.

// To do that, we need to tell our function what the smallest instance looks like.

// If you recall, with proof by induction we need to establish two things:
// - base
// - induction

// Recursion is similar. We also need to establish a base case but rather than induction, we establish the recursive case.

// We use the recursive case to break the problem down into smaller instances.

// We use the base case to return when there are no more problems to be solved.
/* ----------------------------------- END ---------------------------------- */

pub mod print_1_to_n {

    // Forward Recursion
    // Time Complexity: O(N) { Since the function is being called n times, and for each function, we have only one printable line that takes O(1) time, so the cumulative time complexity would be O(N) }
    // Space Complexity: O(N) { In the worst case, the recursion stack space would be full with all the function calls waiting to get completed and that would make it an O(N) recursion stack space }.
    pub fn recursion(n: u32){
        if n == 0 {
            return
        }
        println!("{}",n);
        recursion(n-1);
    }

    // Forward Recursion
    // Time Complexity: O(N) { Since the function is being called n times, and for each function, we have only one printable line that takes O(1) time, so the cumulative time complexity would be O(N) }
    // Space Complexity: O(N) { In the worst case, the recursion stack space would be full with all the function calls waiting to get completed and that would make it an O(N) recursion stack space }.
    pub fn recursion_v2(i:u32,n: u32){
        if i > n {
            return
        }
        println!("{}",i);
        recursion_v2(i+1,n);
    }


    // Backward Recursion
    // Time Complexity: O(N) { Since the function is being called n times, and for each function, we have only one printable line that takes O(1) time, so the cumulative time complexity would be O(N) }
    // Space Complexity: O(N) { In the worst case, the recursion stack space would be full with all the function calls waiting to get completed and that would make it an O(N) recursion stack space }.
    pub fn backtracking(n: u32){
        if n == 0 {
            return
        }
        backtracking(n-1);
        println!("{}",n);
    }

    // Backward Recursion
    // Time Complexity: O(N) { Since the function is being called n times, and for each function, we have only one printable line that takes O(1) time, so the cumulative time complexity would be O(N) }
    // Space Complexity: O(N) { In the worst case, the recursion stack space would be full with all the function calls waiting to get completed and that would make it an O(N) recursion stack space }.
    pub fn backtracking_v2(i:u32, n: u32){
        if i < 1 {
            return
        }
        backtracking_v2(i-1, n);
        println!("{}",i);
    }

}

pub fn print_n_to_1_v1(i:u32, n: u32){
    if i < 1 {
        return
    }
    println!("{}",i);
    print_n_to_1_v1(i-1, n);
}

pub fn print_n_to_1_v2(i:u32, n: u32){
    if i > n {
        return
    }
    print_n_to_1_v2(i+1, n);
    println!("{}",i);
}


// Time Complexity: O(N) { Since the function is being called n times, and for each function, we have only one printable line that takes O(1) time, so the cumulative time complexity would be O(N) }
// Space Complexity: O(N) { In the worst case, the recursion stack space would be full with all the function calls waiting to get completed and that would make it an O(N) recursion stack space }.
pub fn print_text_ntimes(i:u32, n:u32){
    if i > n {
        return
    }
    println!("Wow !!!");

    print_text_ntimes(i+1,n);
}




