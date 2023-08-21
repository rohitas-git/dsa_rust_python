fn quick_sort_lomuto(arr: &mut [u32], l: usize, h: usize) {
    if l < h {
        let p = parition::lomuto_parition(arr, l, h);
        quick_sort_lomuto(arr, l, p-1);
        quick_sort_lomuto(arr, p+1, h)
    }
}

mod parition {
    pub fn lomuto_parition(arr: &mut [u32], l: usize, h: usize) -> usize {
        let pivot = arr[h];
        let mut i = l; // i=l-1 not possible due to usize type and when l=0

        // for smaller elements, swap them with first elements of (>=) Parition
        // and increase ptr to last elemenet of (<) +1
        // for larger elements, there is already fit for(>=) Parition; +1 the ptr to its last element
        for j in l..h {
            if arr[j] < pivot {
                // after addition of new element in < parition, size+1 i.e i+=1
                i = i + 1;
                // swap curr element with first element of >= parition
                (arr[i - 1], arr[j]) = (arr[j], arr[i - 1]);
            }
        }

        // place the pivot (last element) in its correct position
        (arr[i], arr[h]) = (arr[h], arr[i]);
        return i;
    }
}

#[cfg(test)]
mod test_quick_sort_lomuto {
    use super::*;

    #[test]
    fn quick_sort_lomuto_works() {
        let mut arr = [10,110,30,90,20,50];
        quick_sort_lomuto(&mut arr, 0, 5);
        assert_eq!(arr, [10,20,30,50,90,110] );
    }
}