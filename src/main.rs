use std::env;
use std::fs::{
    self,
    DirEntry,
};
use std::path::PathBuf;



// take terminal input
fn term_input() -> PathBuf{
    let inp: Vec<String> = env::args().collect();
    let dir_name:PathBuf = if inp.len() < 2 || inp[2] == "."{
        env::current_dir().unwrap()
    }else{
        PathBuf::from(&inp[2])
    };
    return dir_name; 
}





// get vector of all entries in the directory input
fn get_dir_entries(dir_path:&PathBuf)->Vec<DirEntry>{
    let dir_content_result = fs::read_dir(dir_path);
    let dir_entries = match dir_content_result{
        Ok(entries) => {
            let mut entry_vec = Vec::new();
            for entry in entries {
                match entry {
                    Ok(val) => entry_vec.push(val),
                    Err(err) => println!("Error reading entry: {}", err),
                }
            }

            entry_vec
        },
        Err(err) => {
            println!("Error getting dir entries : {}", err);
            Vec::new()
        }
    };

    dir_entries
}

fn main(){
    let dir_path = term_input();
    let dir_entries = get_dir_entries(&dir_path);
    let dir_iter = dir_entries.iter();

    for dir in dir_iter {
        println!("{:?}", dir.file_name());
    }
}