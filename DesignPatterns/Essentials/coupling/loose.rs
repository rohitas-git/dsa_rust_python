// INTERFACE:
// A contract that should specify the capabilities the a  class should provide

/* ------------------------------------ x ----------------------------------- */

trait Chef{
    fn chef_rating() i32;
}

struct Indian;
struct Chinese;

impl Player for Indian{
    fn chef_rating() i32{
        5
    }
}

impl Player for Chinese{
    fn chef_rating() i32{
        4
    }
}


fn main() {
    let raj = Indian;
    let zhang = Chinese;

    raj.chef_rating();
    zhang.chef_rating();
}