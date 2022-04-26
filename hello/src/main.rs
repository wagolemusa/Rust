#[derive(Debug)]
fn main(){
    
    enum TYPE{
        INTEGER(i32),
        STRING(String),
    }

    let mut v2 = Vec::new();
    v2.push(TYPE::INTEGER(3));
    v2.push(TYPE::STRING(String::from("ok")));
    
    println!("{:?}", v2);

    match &v2[0]{
        TYPE::INTEGER(i) => println!("{}", i),
        TYPE::STRING(s) => println!("{}", s),
    }
}
