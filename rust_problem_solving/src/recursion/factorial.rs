pub mod caluclate_factorial {

    // time O(n) , space O(1)
    pub fn iterative(n: u32) {
        let mut res = 1;
        for i in 1..=n {
            res = res * i;
        }
        println!("{}", res);
    }

    // time O(n) , space O(n)
    pub fn recursive(n: u32) -> u32 {
        if n == 1 {
            return 1;
        }

        n * recursive(n - 1)
    }
}
