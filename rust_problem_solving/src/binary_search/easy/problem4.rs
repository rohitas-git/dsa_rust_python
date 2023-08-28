/* --------------------- Floor and Ceil in Sorted Array --------------------- */
// Problem Statement: You’re given an sorted array arr of n integers and an integer x. 
// Find the floor and ceiling of x in arr[0..n-1].
// 
// The floor of x is the largest element in the array which is smaller than or equal to x.
// The ceiling of x is the smallest element in the array greater than or equal to x.

// floor of x ->  arr[i] <= x
// ceiling of x -> x <= arr[j]  - lower bound

/* ------------------------------------ x ----------------------------------- */

fn get_floor_ceiling(arr:&[u32], target:u32) -> (Option<u32>,Option<u32>){
    let floor = get_floor(arr, target);
    let ceiling = get_ceiling(arr, target);
    (floor,ceiling)
}


fn get_floor(arr:&[u32], target:u32) -> Option<u32>{

    let mut answer = None;
    let n = arr.len();
    let mut low = 0;
    let mut high = n - 1;

    while low <= high {
        let mid = low + (high - low) / 2;
        if arr[mid] <= target {
            answer = Some(mid);
            low = mid.checked_add(1).unwrap_or(0);
        } else {
            high = mid.checked_sub(1).unwrap_or(0);
            if high == 0{
                break;
            }
        }
    }
    if let Some(answer) = answer {
        Some(arr[answer])
    }
    else {
        None
    }
}

#[cfg(test)]
mod test_floor{
    use super::*;
    #[test]
    fn floor_works() {
        let arr = [3, 4, 4, 7, 8, 10];
        
        let res = get_floor(&arr, 8);
        assert_eq!(res, Some(8));
    }
}

fn get_ceiling(arr:&[u32], target:u32) -> Option<u32>{
    let mut answer = None;
    let n = arr.len();
    let mut low = 0;
    let mut high = n - 1;

    while low <= high {
        let mid = low + (high - low) / 2;
        if arr[mid] >= target {
            answer = Some(mid);
            high = mid.checked_sub(1).unwrap_or(0);
            if high == 0{
                break;
            }
        } else {
            low = mid.checked_add(1).unwrap_or(0);
            if low == n-1{
                answer = Some(low)
            }
        }
    }
    if let Some(answer) = answer {
        Some(arr[answer])
    }
    else {
        None
    }
}

#[cfg(test)]
mod test_ceiling{
    use super::*;

    #[test]
    fn ceiling_works() {
        let arr = [3, 4, 4, 7, 8, 10];
        
        let res = get_ceiling(&arr, 5);
        assert_eq!(res, Some(7));
    }
}

#[cfg(test)]
mod test_getting_floor_ceiling_of_array {
    use super::*;

    #[test]
    fn floor_ceiling_works() {
        let arr = [3, 4, 4, 7, 8, 10];        
        let res = get_floor_ceiling(&arr, 6);
        assert_eq!(res, (Some(4),Some(7)));

        let arr= [1, 3, 4, 4, 7, 8, 10];
        let res = get_floor_ceiling(&arr, 8);
        assert_eq!(res, (Some(8),Some(8)));
        let res = get_floor_ceiling(&arr, 4);
        assert_eq!(res, (Some(4),Some(4)));
    }
}