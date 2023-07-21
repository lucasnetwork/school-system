use std::fs;
use std::io;
use argon2::
    {Argon2,PasswordHash};
use password_hash::{PasswordHasher,SaltString,PasswordVerifier,Encoding};
mod dashboard;
mod verify_if_name_exist;

fn main() {
   loop{
        println!("Hello,Digit 0 to login, 1 to register and 2 to exit");
        let mut digit = String::new();
        io::stdin().read_line(&mut digit).expect("error");
        let digit_number:u32 = digit.trim().parse().expect("This is not number");
        match digit_number{
            0 => match login(){
                Err(err)=>println!("{}",err),
                Ok(_)=>dashboard::dashboard()

            },
            1 => {
                match register(){
                    Err(err)=>println!("{}",err),
                    Ok(_)=>println!("correct"),
                }
            },
            2 => break,
            _ => panic!("Not exist")
        }

    }
}


fn login()->Result<bool,String>{
    println!("Digit your name");
    let mut name = String::new();
    let mut password = String::new();
    io::stdin().read_line(&mut name).expect("error in get name");
    println!("Digit your password");
    io::stdin().read_line(&mut password).expect("error in get password");
    let  file = fs::read_to_string("test.txt").expect("LogRocket: Should have been able to read the file");
    let file_splited:Vec<&str> = file.split(";").collect();
    let mut userExist = String::new();
    for user in file_splited {
        if user.contains(name.trim()){
            userExist = user.to_string();
            break;
        };
    };
    let user_array:Vec<&str> = userExist.split(" ").collect();
    let password_hash = user_array[1];
    let argon2 = Argon2::default();
    let pass_encoded= PasswordHash::parse(password_hash,Encoding::B64).unwrap();
    match argon2.verify_password(password.as_bytes(),&pass_encoded ){
        Ok(_)=>Ok(true),
        Err(err)=> Err(err.to_string())
    }

}


fn register() -> Result<bool,String>{
    println!("Digit your name");
    let mut name = String::new();
    let mut password = String::new();
    io::stdin().read_line(&mut name).expect("error in get name");
    println!("Digit your password");

    io::stdin().read_line(&mut password).expect("error in get password");
    match verify_if_name_exist::verify_if_user_exist(&name,"test.txt".to_string()){
        Err(_)=>{return Err("User alread exist".to_string())},
        Ok(_)=>{}
    };
    let password_hash = encrypt_password(password);
    save_user_in_file(&name,password_hash,"test.txt".to_string());
    Ok(true)

}

fn save_user_in_file(name:&String,password:String,path:String){
    let file = fs::read_to_string(&path).expect("LogRocket: Should have been able to read the file");
    let file_trim = file;

    let new_user =format!("{} {};",&name.trim(),password).to_string();
    let file_string =
        file_trim+&new_user;
    fs::write(path,file_string).unwrap();
}


fn encrypt_password(password:String)-> String{
    let salt = SaltString::generate(&mut rand::thread_rng());
    let argon2 = Argon2::default();

 
    let password_hash = argon2.hash_password(password.as_bytes(), &salt).expect("msg").to_string();
    password_hash
}