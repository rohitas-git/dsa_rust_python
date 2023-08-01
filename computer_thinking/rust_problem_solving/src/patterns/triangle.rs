pub fn print_trianglev1(n:u32)
{
    for i in 0..n {
        let space = n -i -1;
        let star = 2*i +1;

        for _j in 0..space{
            print!("  ");
        }
        for _j in 0..star{
            print!("* ");
        }
        for _j in 0..space{
            print!("  ");
        }
        println!()
    }
}

pub fn print_trianglev2(n:u32) {
    for i in 0..n {
        let space = i;
        let star = 2*n - 2*i -1 ;

        for _j in 0..space{
            print!("  ");
        }
        for _j in 0..star{
            print!("* ");
        }
        for _j in 0..space{
            print!("  ");
        }
        println!()
    }
}

pub fn print_trianglev3(n:u32){
    print_trianglev1(n);
    print_trianglev2(n);
}

pub fn print_trianglev4(n:u32){
    for i in 0..n{
        let space = n-1 -i;
        let star = i+1;
        
        for _j in 0..star{
            print!("* ");
        }
        for _j in 0..space{
            print!(" ");
        }
        println!();
    }

    for i in (0..n).rev(){
        let space = n-1 -i;
        let star = i+1;
        
        for _j in 0..star{
            print!("* ");
        }
        for _j in 0..space{
            print!(" ");
        }
        println!();
    }
}


pub fn print_trianglev5(n:u32){

    for i in (0..(n+1)).rev(){
        //total chars = 4*n-1

        let nums = n - i;
        let space = i*4;

        for j in 0..nums{
            print!("{} ",j+1);
        }
        for _j in 0..space{
            print!(" ");
        } 
        for j in (0..nums).rev(){
            print!("{} ",j+1);
        } 
        println!(); 
    }

}

