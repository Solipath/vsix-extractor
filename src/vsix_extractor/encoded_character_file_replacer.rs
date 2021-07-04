use std::{fs::{self}, path::{Path, PathBuf}};


pub struct EncodedCharacterFileReplacer;

impl EncodedCharacterFileReplacer{
    pub fn new()-> Self{
        Self{}
    }

    fn replace_encoded_character_retrieve_path(&self, single_file_path: &Path) -> PathBuf{
        let file_name = single_file_path.file_name().unwrap().to_str().unwrap();
        if file_name.contains("%") {
            let new_file_name = urlencoding::decode(file_name).unwrap();
            let mut new_file_path = single_file_path.parent().unwrap().to_path_buf();
            new_file_path.push(new_file_name);
            fs::rename(single_file_path.clone(), new_file_path.clone()).expect("failed to rename");
            new_file_path.to_path_buf()
        } else {
            single_file_path.to_path_buf()
        }
    }

    pub fn recursively_replace_encoded_characters(&self, path: &PathBuf) {
        crate::recursive_file_looper::recursively_loop_through_files(path, &|each_entry| 
                self.replace_encoded_character_retrieve_path(each_entry)
        );
    }
}

#[cfg(test)]
mod test{

    use super::*;
    use std::fs::{File, create_dir_all};
    use tempfile::tempdir;

    #[test]
    fn changes_nothing_if_file_name_does_not_contain_special_characters(){
        let  temp_path = tempdir().unwrap().path().to_path_buf();
        let mut path_to_add = temp_path.clone();
        path_to_add.push("directory name");
        create_dir_all(path_to_add.clone()).unwrap();

        let file_replacer = EncodedCharacterFileReplacer::new();

        file_replacer.recursively_replace_encoded_characters(&temp_path);

        assert!(path_to_add.exists());
    }

    #[test]
    fn changes_percent20_to_space(){
        let  temp_path = tempdir().unwrap().path().to_path_buf();
        let mut path_to_add = temp_path.clone();
        path_to_add.push("directory%20name");
        create_dir_all(path_to_add.clone()).unwrap();
        let mut expected_path = temp_path.clone();
        expected_path.push("directory name");
        let file_replacer = EncodedCharacterFileReplacer::new();

        file_replacer.recursively_replace_encoded_characters(&temp_path);

        assert!(!path_to_add.exists());
        assert!(expected_path.exists());
    }

    #[test]
    fn move_files_inside_encoded_directory(){
        let  temp_path = tempdir().unwrap().path().to_path_buf();
        let mut path_to_add = temp_path.clone();
        path_to_add.push("directory%20name");
        create_dir_all(path_to_add.clone()).unwrap();
        let mut file_to_add = path_to_add.clone();
        file_to_add.push("file.txt");
        File::create(file_to_add).unwrap();
        let mut expected_path = temp_path.clone();
        expected_path.push("directory name");
        let mut expected_file = expected_path.clone();
        expected_file.push("file.txt");
        let file_replacer = EncodedCharacterFileReplacer::new();

        file_replacer.recursively_replace_encoded_characters(&temp_path);

        assert!(!path_to_add.exists());
        assert!(expected_path.exists());
        assert!(expected_file.exists());
    }

    #[test]
    fn can_update_encoded_file_inside_directory(){
        let  temp_path = tempdir().unwrap().path().to_path_buf();
        let mut path_to_add = temp_path.clone();
        path_to_add.push("directory name");
        create_dir_all(path_to_add.clone()).unwrap();
        let mut file_to_add = path_to_add.clone();
        file_to_add.push("file%23.txt");
        File::create(file_to_add.clone()).unwrap();
        let mut expected_file = path_to_add.clone();
        expected_file.push("file#.txt");
        let file_replacer = EncodedCharacterFileReplacer::new();

        file_replacer.recursively_replace_encoded_characters(&temp_path);

        assert!(path_to_add.exists());
        assert!(!file_to_add.exists());
        assert!(expected_file.exists());
    }
    #[test]
    fn move_files_inside_encoded_directory_with_encoded_file(){
        let  temp_path = tempdir().unwrap().path().to_path_buf();
        let mut path_to_add = temp_path.clone();
        path_to_add.push("directory%20name");
        create_dir_all(path_to_add.clone()).unwrap();
        let mut file_to_add = path_to_add.clone();
        file_to_add.push("file%23.txt");
        File::create(file_to_add.clone()).unwrap();
        let mut expected_path = temp_path.clone();
        expected_path.push("directory name");
        let mut expected_file = expected_path.clone();
        expected_file.push("file#.txt");
        let file_replacer = EncodedCharacterFileReplacer::new();

        file_replacer.recursively_replace_encoded_characters(&temp_path);

        assert!(!path_to_add.exists());
        assert!(!file_to_add.exists());
        assert!(expected_path.exists());
        assert!(expected_file.exists());
    }

}