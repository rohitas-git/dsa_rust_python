/* ------------------------- Hoare's parition Scheme ------------------------ */

// In this, we consider the first element as pivot

// - It doesn't fix the pivot in contrast to Lomuto scheme
//       where pivot is fixed at last element during process and at end, we also get pivot position
// - In genral, Hoare > Lomuto parition scheme
// - Both are not Stable algorithm except Naive algorithm for parition

// [< Pivot][Unknown??][>= Pivot]
// l________i_________j___________h
//          -->      <--

// It feels like we are position the pivot from both direction
// by making use of rule that leftItem < pivot and rightItem > pivot
// On the way, if we violation of this, we swap them for correction
// Repeat till both direction ptrs cross each other

//  time - O(n)
//  space - O(1)

fn hoare_parition(arr: &mut [u32], l: usize, h: usize) -> usize {
    // pivot - first element of array
    let pivot = arr[0];

    let mut i = l;
    let mut j = h;

    loop {
        while arr[i] < pivot {
            i += 1;
        }
        while arr[j] > pivot {
            j -= 1;
        }
        // when i, j violate the above rules, then run below code

        // if i and j cross each other, return j i.e last item index of [<=] Parition
        if i >= j {
            return j;
        }

        // swap the violating i and j
        (arr[i], arr[j]) = (arr[j], arr[i]);
        j -= 1;
        i += 1;
    }
}

#[cfg(test)]
mod test_hoare_pariiton {
    use super::*;

    #[test]
    fn hoare_works() {
        let mut arr = [7, 1, 2, 5, 10, 8, 9, 21];
        let n = arr.len();
        assert_eq!(hoare_parition(&mut arr, 0, n - 1), 2);
        println!("{:?}", arr);
    }
}
