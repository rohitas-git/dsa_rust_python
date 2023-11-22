
/* ------------------------ Basic Recursion Problems ------------------------ */

// Print Name 5 times
// Print linearly from 1 to N
// Print linearly from N to 1
// Print linearly from 1 to N (by Backtracking)
// Print linearly from N to 1 (by Backtracking)

/* ------------------------------------ x ----------------------------------- */

fn print_linearly(n:u32){

    // print_range(1, n);
    // print_range_backtrack(1, n);
    print_from(n, 1);
    print_from_backtrack(n, 1);

}

fn print_name(){
    print_name_5(5);
}

fn print_from(start: u32, end: u32){
    if start < end{
        println!();
        return;
    } 
    print!("{}-> ", start);
    print_from(start-1, end);
}

fn print_from_backtrack(start: u32, end: u32){
    if start < end{
        println!();
        return;
    } 
    print_from_backtrack(start, end+1);
    print!(" <-{}", end);
}

fn print_range(start:u32, end: u32){
    if start > end{
        println!();
        return;
    } 
    print!("{} ->", start);
    print_range(start+1, end);
}

fn print_range_backtrack(start:u32, end: u32){
    if end < start {
        println!();
        return;
    }
    print_range_backtrack(start, end-1);
    print!("<-{} ", end);
}



fn print_name_5(mut count: u8){
    if count == 0{
        return
    }
    println!("Goku");
    count-=1;
    print_name_5(count);
}

#[cfg(test)]
mod test_basics {
    use super::*;

    #[test]
    fn test_print_linearly() {
        print_linearly(6);
    }

    #[test]
    fn test_print_name() {
        print_name();
    }
}