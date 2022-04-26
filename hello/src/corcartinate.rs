fn main(){
    let h = String::from("Hello, ");
    let w = String::from("world");
    let s = h + &w;
    println!("{}", s);
}