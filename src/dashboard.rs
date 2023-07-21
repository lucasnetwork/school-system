use std::io;
use std::fs;

 use crate::verify_if_name_exist;

#[derive(Clone)]
struct Student{
    name:String,
}

struct Class{
    name:String,
    students:Vec<Student>
}


fn save_user_in_file(name:&String,students:&String,path:String){
    let file = fs::read_to_string(&path).expect("LogRocket: Should have been able to read the file");
    let file_trim = file;


    let new_classe_with_trim =format!("{};",&name.trim()).to_string();
    let students_with_trim =format!("{};",&students.trim()).to_string();
    let file_string =
        file_trim+&new_classe_with_trim + ":" + &students_with_trim;
    fs::write(path,file_string).unwrap();
}

impl Class{
    fn save(name:String,students:String){
        let studentsArray: Vec<Student> ={
            let strings = students.split(",");
            let stringsLength = strings.clone().count();
            println!("{}",stringsLength);
            let mut array: Vec<Student> = Vec::with_capacity(stringsLength);
                
    
            for student in strings {
                array.push(Student {
                    name:student.to_string()}
                )
            }
            array
        };
        let class = Class { name, students: studentsArray };
        save_user_in_file(&class.name,&students,"classes.txt".to_string());
    }
}


pub fn dashboard(){
    loop{
        println!("Hello,Digit 0 to register class\n1 to register student\n 2 to get students\n 3 to get classes\n 4 to exit");
        let mut digit = String::new();
        io::stdin().read_line(&mut digit).expect("error");
        let digit_number:u32 = digit.trim().parse().expect("This is not number");
        match digit_number{
            0 =>    match register(){
                Err(err)=>println!("{}",err),
                Ok(_)=>println!("correct"),
            },
            // 1 => {
             
            // },
            2 => break,
            _ => panic!("Not exist")
        }

    }

}

fn register() -> Result<bool,String>{
    println!("Digit a name of classe");
    let mut class_name = String::new();
    io::stdin().read_line(&mut class_name).expect("error in get name");
    println!("Digit the names of students,separate by ,");
    let mut students = String::new();
    io::stdin().read_line(&mut students).expect("error in get name");
   
    match verify_if_name_exist::verify_if_user_exist(&class_name,"classes.txt".to_string()){
        Err(_)=>{return Err("User alread exist".to_string())},
        Ok(_)=>{}
    };
    Class::save(class_name,students);
    Ok(true)
}
