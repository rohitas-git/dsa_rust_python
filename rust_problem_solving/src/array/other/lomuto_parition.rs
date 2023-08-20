// Parition the array according to a given parition point (pivot)
// such that elements smaller than ppValue are to left it
// and larger elements to right of it

// Example:
// arr = [10,50,30,90,20,110] and parition point = 1
// result: arr = [10,30,20, 50 ,110,90]

// Lomuto Parition:
// Takes last element as pivot (any pivot is possible; here it's used for simplicity)
// Will be used in quicksort recursively, therefore have l and h in fn args

// [< Pivot][>= Pivot][Unprocessed] 
// l________i-1_________j___________h 

// element at i is >= pivot (first element of >= parition)
// element at j is unprocessed 
fn lomuto_parition(arr:&mut [u32], l: usize, h:usize)-> usize{
    
    let pivot = arr[h];
    let mut i = l;      // i=l-1 not possible due to usize type and when l=0

    // for smaller elements, swap them with first elements of (>=) Parition 
    // and increase ptr to last elemenet of (<) +1
    // for larger elements, there is already fit for(>=) Parition; +1 the ptr to its last element
    for j in l..h{
        if arr[j] < pivot{
            // after addition of new element in < parition, size+1 i.e i+=1
            i = i+1;
            // swap curr element with first element of >= parition
            (arr[i-1],arr[j])= (arr[j],arr[i-1]);
        }
    }

    // place the pivot (last element) in its correct position
    (arr[i],arr[h]) = (arr[h],arr[i]);
    return i;
}


#[cfg(test)]
mod test_pivot_paritioning {
    use super::*;

    #[test]
    fn lomuto_parition_works() {
        let mut arr = [10,110,30,90,20,50];
        assert_eq!(lomuto_parition(&mut arr, 0, 5), 3);
        println!("{:?}",arr);
    }
}


