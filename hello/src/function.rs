fn main() {
    println!("Hey Hey");
    my_func();
    my_func2(45);
    let mydata = my_func3();
    let newdata = my_func4();

}

fn my_func(){
    println!("Hello musa");
}

fn my_func2(age:u32){
    println!("my age is {}", age);
}

fn my_func3() -> i32{
    return 33;
}

fn my_func4() -> i32{
    println!("my age");
    23
}
