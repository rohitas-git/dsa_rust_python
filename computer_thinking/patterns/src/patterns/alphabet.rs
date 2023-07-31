pub fn print_charv1(n:u32){
    for i in 0..n{
        for _j in 0..i{
            print!("{} ", (b'A'+ i as u8 -1u8) as char);
        }
        println!();
    }
}

pub fn print_charv2(n:u32){
    for i in 0..n {
        let space = n -i -1;
        let star = 2*i +1;

        for _j in 0..space{
            print!("  ");
        }
        for _j in 0..star{
            let this = (b'A' + (1*_j*(2*i-_j)) as u8) as char ;
            print!("{} ", this);
        }
        for _j in 0..space{
            print!("  ");
        }
        println!()
    }

}

pub fn print_charv3(n:u32){
    for i in 0..n {
        let space = n -i -1;
        let star = 2*i +1;

        for _j in 0..space{
            print!("  ");
        }
        for _j in 0..star{
            let this = (b'A' + (1*_j*(2*i-_j)) as u8) as char ;
            print!("{} ", this);
        }
        for _j in 0..space{
            print!("  ");
        }
        println!()
    }

}