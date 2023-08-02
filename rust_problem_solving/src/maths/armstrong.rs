
// Time: theta(log N) where N is the number, log N is number of digits
// Space: theta(1)
pub fn is_armstrong(mut n: u32) {
    let original = n;
    let mut num = n;
    let mut count = 0u32;
    let mut sum = 0u32;
    const BASE_10: u32 = 10;

    while n > 0 {
        n = n / BASE_10;
        count +=1;
    }

    while num > 0{
        let digit = num % BASE_10;
        sum = sum + digit.checked_pow(count).expect("Shouldn't overflow when calculating sum");
        num = num / BASE_10;
    }

    match sum == original {
        true => println!("Yes, it's an Armstrong number"),
        false => println!("No, it's not an Armstrong number")
    }
}
