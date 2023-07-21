use std::fs;

fn Verify_if_file_exist(path:&String)->bool{
    match fs::read_to_string(path){
        Err(_)=>{
            fs::write(path, "");
            true
        },
        Ok(_)=>{true}
    }
}
pub fn verify_if_user_exist(name:&String,path_file:String)->Result<bool,bool>{
    Verify_if_file_exist( &path_file);
    let  file = fs::read_to_string(path_file).expect("LogRocket: Should have been able to read the file");
    let file_splited:Vec<&str> = file.split(";").collect();

    for user in file_splited {
        if user.contains(name.trim()){
            return Err(false)
        }
    };
    Ok(true)
}