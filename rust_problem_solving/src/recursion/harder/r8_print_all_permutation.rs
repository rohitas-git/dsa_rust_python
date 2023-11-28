
/* ---------------- Print all permutations of a string/array ---------------- */

// Two solution - Both recursive

// Solution 1 (has extra space complexity):  time O(N!), space O(N)
// Use data structure to store one possible permutation
// Another map/ds to tick which elements are picked


/* ------------------------------- Permutation ------------------------------ */
// Array has N distinct elements => N! permutations possible


/* ------------------------------------ x ----------------------------------- */
type Permutation = Vec<u32>;
fn solve_permutations(arr: &[u32], store:&mut Permutation, ans: &mut Vec<Permutation>,  chosen: &mut Vec<bool>){

    if store.len() == arr.len(){
        ans.push(store.to_owned());
        println!("{}: {:?}",ans.len()-1, store);
        return;
    }

    for i in 0..arr.len() {
        if !chosen[i]{
            chosen[i] = true;
            store.push(arr[i]);
            solve_permutations(arr, store, ans, chosen);
            store.pop();
            chosen[i] = false;
        }
    }
}


/* ---------------------------------- Tests --------------------------------- */
#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_solution_1() {
        let arr = &[1u32,2,3];
        solve_permutations(arr, &mut vec![], &mut vec![vec![]], &mut vec![false;arr.len()]);
    }
}