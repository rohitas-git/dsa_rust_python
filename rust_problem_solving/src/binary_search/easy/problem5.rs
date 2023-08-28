/* -------------------- Last occurrence in a sorted array ------------------- */
// Given a sorted array of N integers, 
// write a program to find the index of the last occurrence of the target key. 
// If the target is not found then return -1.

// Input: N = 7, target=13, array[] = {3,4,13,13,13,20,40}
// Output: 4

/* ------------------------------------ x ----------------------------------- */
// Approach:
// Have variable to store last occurence of target value and do normal binary search for it


fn last_occurrence(arr:&[u32], target:u32) -> Option<usize>{
    let n = arr.len();
    let mut low = 0;
    let mut high = n-1;
    let mut last_occurrence = None;

    while low <= high{
        let mid = low +(high - low)/2;

        if arr[mid] == target {
            last_occurrence = Some(mid);
            low = mid+1;
        }
        else if arr[mid] < target {
            low = mid+1;
        }
        else {
            high = mid-1;
            
        }
    }
    last_occurrence
}

#[cfg(test)]
mod test_getting_last_occurrence {
    use super::*;

    #[test]
    fn test_solution() {
        let arr =[3,4,13,13,13,20,40];
        
        let res = last_occurrence(&arr, 13);
        assert_eq!(res, Some(4));
    
        let res = last_occurrence(&arr, 60);
        assert_eq!(res, None);
    }
}