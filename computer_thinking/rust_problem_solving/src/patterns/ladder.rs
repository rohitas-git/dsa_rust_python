pub fn print_ladderv1(n:u32){
    for i in 0..n {
        for _j in 0..(i+1) {
            print!("* ");
        }
        println!();
    }
}

pub fn print_ladderv2(n:u32){
    for i in 0..n {
        let num = i+1;
        for _j in 0..(i+1) {
            print!("{} ",num );
        }
        println!();
    }
}

pub fn print_ladderv3(n:u32){
    for i in 0..n {
        let mut num = 1;
        for _j in 0..(i+1) {
            print!("{} ",num );
            num+=1;
        }
        println!();
    }
}

pub fn print_ladderv4(n:u32){
    let mut start=1; 
    for i in 0..n {
        let mut here = start;
        for _j in 0..(i+1) {
            print!("{} ", here);
            here = (here+1)%2;
        }
        println!();
        start = (start + 1)%2;
    }
}