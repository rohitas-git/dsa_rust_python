pub mod sum_n_natural_numbers{

    // time: O(n)
    // space: O(n)
    pub fn parameterized_way(sum:u32,n:u32){
        if n == 0{
            println!("Sum: {}", sum);
            return 
        }
        parameterized_way(sum+n,n-1);
    }

    // time: O(n)
    // space: O(n)
    pub fn functional_way(n:u32)-> u32{
    
        if n == 0{
            return 0
        }

        return n + functional_way(n-1);
    }

    pub fn using_formula(n:u32){}
    pub fn using_loop(n:u32){}
}