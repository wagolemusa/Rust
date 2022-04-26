fn main(){
    let mut v : Vec<i32> = Vec::new();
    
    v.push(5);
    v.push(2);
    v.push(4);
    v.push(8);
    
    println!("{:?}", &v);
    v.pop();
    println!("{:?}", &v);
    v.remove(0);
    println!("{:?}", &v);
    
    // Get Values
    println!("{:}", &v[0]);
    v.push(10);
    v.push(20);
    v.push(13);
    println!("{:?}", &v);
    
    for i in &v {
        println!("{}", i);
    }
}
