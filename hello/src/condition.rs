fn main() {
    sum(56)

}
    fn sum(number: u32){
    
        if number % 4 == 0 {
            println!("Number is not divisible by 4");
        } else if  number % 3 == 1 {
            println!("Number is is divisible by 3");
        } else {
            println!("number does'nt divide with any of these")
        }
    }