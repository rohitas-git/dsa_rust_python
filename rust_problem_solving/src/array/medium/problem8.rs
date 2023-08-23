/* --------------------------- Leaders in an Array -------------------------- */
// Problem Statement: 
// Given an array, print all the elements which are leaders. 
// A Leader is an element that is greater than all of the elements on its right side in the array.

// Input: arr = [4, 7, 1, 0]
// Output: 7 1 0


fn brute(arr:&[u32]){
    let mut leaders: Vec<u32> = vec![];
    let n = arr.len();

    for i in 0..n{
        let mut leader = true;

        for j in (i+1)..n{
            if arr[j] > arr[i]{
                leader = false;
                break;
            }
        }
        if leader {
            leaders.push(arr[i]);
        }
    }
    println!("{:?}",leaders);
}

// time - O(N), space - O(N)
fn optimal(arr:&[u32]){
    let n = arr.len();
    let mut leaders: Vec<u32> = vec![];
    let mut bigger = 0;

    leaders.push(arr[n-1]);
    bigger = arr[n-1];
    

    for i in (0..(n-1)).rev(){
        if arr[i] > bigger{
            bigger = arr[i];
            leaders.push(bigger);
        }
    }
    println!("{:?}",leaders);

}

#[cfg(test)]
mod test_finding_leaders {
    use super::*;

    #[test]
    fn optimal_ok() {
       let arr = [10, 22, 12, 3, 0, 6];
       optimal(&arr); 
    }
}