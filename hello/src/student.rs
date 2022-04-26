use std::io;
fn main(){
    
    let stud = Strudent {
        passing_grade: 70,
        failing_grade: 40,
    };
    
    // println!("Results are {}", );
    loop {
        println!("Input the grade");
        let mut grademarks = String::new();

        io::stdin()
            .read_line(&mut grademarks)
            .expect("Failed to read the line");

        let grademarks: u32 = match grademarks.trim().parse(){
            Ok(num) => num,
            Err( _ ) => continue,
        };  

        stud.grade(grademarks);

    }
 
}

 struct Strudent {
        passing_grade: u32,
        failing_grade: u32,
    }

    impl Strudent{

        fn grade(&self, grademark: u32){
            if self.passing_grade > grademark {
                println!("You have Passed the Exams");
            }else if self.failing_grade < grademark {
                println!("You have to Repeat the Exams");
            } else {
                println!("Go back to class one")
            }
            
        }
    }
    

