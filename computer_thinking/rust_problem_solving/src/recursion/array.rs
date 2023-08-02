pub mod reverse_array{

    // time O(n) space O(1)
    pub fn rusty(arr: &[u32]) {
        // for num in arr.iter().rev() {
        //     println!("{}", num);
        // }
        let l = arr.len();
        for i in 0..l {
            println!("{}", arr[l - 1 - i]);
        }
    }

    pub fn by_swapping(arr: &mut [u32]){
        
        let ll: usize = arr.len();
        let l: usize = arr.len() / 2;

        for i in 0..l{
            let tmp = arr[i];
            arr[i] = arr[ll-1-i];
            arr[ll-1-i] = tmp;
        }

        for i in 0..ll {
            println!("{}", arr[i]);
        }
    }

    pub fn recursive(arr: &mut [u32], start: usize, end: usize){
        
        if start < end {
            let tmp = arr[start];
            arr[start] = arr[end];
            arr[end] = tmp;
            recursive(arr, start + 1, end - 1);
        } 
        else {
            let ll: usize = arr.len();
            for i in 0..ll {
                println!("{}", arr[i]);
            }
        }

    }


    // Not able to create an array of unknown size at runtime
    fn error_creating_extra_array(arr: &[u32]) {
        let l: usize = arr.len();
        // let new_array = <[u32;l]>::try_from(arr).unwrap();
        // let new_array: [u32; l] = arr.try_into().unwrap();

        let vector = arr.to_vec(); // or arr.iter().collect()
        let boxed_slice = vector.into_boxed_slice();
        // let new_array: [u32; l] = boxed_slice.into();

        
        // for i in new_array.iter().rev() {
        //     println!("{}", i);
        // }
    }

}






