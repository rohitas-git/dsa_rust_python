
pub fn is_palindromev1(x: u32) {
    
    let mut tmp = x;
    let mut s: u32 = 0;
    while tmp != 0{
        s = s*10 + tmp%10;
        tmp /=10;
    }
    println!("{}",x == s);
}


pub fn is_palindromev2(x: u32) {
    println!("{}", x.to_string().chars().rev().eq(x.to_string().chars())  );
}

pub fn is_palindromev3(x: i32) -> bool {
    let x = x.to_string();
    let n = x.len();
    let x = x.chars().collect::<Vec<char>>();
    let head = x[..n/2].iter();
    let tail = x[n/2..].iter().rev();
    head.zip(tail).all(|(a, b)| a==b)
}
