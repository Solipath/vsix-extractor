use std::path::{Path, PathBuf};

///Recursively loops through files, and accepts a function to execute on each file
///this function accepts a path for the current file being recursed through, and should
///return the same path, unless this file has been renamed in which case it should return
///the renamed file
pub fn recursively_loop_through_files(path: &Path, function: &dyn Fn(&Path)-> PathBuf){
    for entry in path.read_dir().expect(&format!("directory doesn't exist: &{:?}", path.clone())){
        let updated_path = function(&entry.expect("failed to read entry").path());
        if updated_path.is_dir(){
            recursively_loop_through_files(&updated_path, function);
        }
    }
}