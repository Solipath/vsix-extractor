use std::{fs::{create_dir_all, read_to_string}, path::Path};

use fs_extra::dir::{CopyOptions, copy};
use strip_bom::StripBom;

use super::manifest::Manifest;

pub struct VsixExtractedFileMover;

impl VsixExtractedFileMover{
    pub fn new()-> Self{
        Self{}
    }
    pub fn copy_vsix_contents_to_directory(&self, source_path: &Path, target_path: &Path){
        let mut manifest_file = source_path.to_path_buf();
        manifest_file.push("manifest.json");
        let manifest_content = read_to_string(manifest_file)
            .expect("failed to read file");
        let manifest: Manifest = serde_json::from_str(&manifest_content.strip_bom())
            .expect(&format!("failed to parse: manifest.json"));
        if manifest.get_extension_dir().is_some(){
            self.copy_for_with_extension_dir(source_path, target_path, &manifest);
        } else {
            self.copy_for_no_extension_dir(source_path, target_path);
        }
    }

    fn copy_for_with_extension_dir(&self, source_path: &Path, target_path: &Path, manifest: &Manifest){
        let relative_path = manifest.get_extension_dir().unwrap();
        let relative_path = relative_path.replace("[installdir]", target_path.to_str().unwrap());
        let new_target_path = Path::new(&relative_path);
        create_dir_all(new_target_path.clone()).expect("failed to create target directories");
        self.copy_files(source_path, new_target_path);
    }

    fn copy_for_no_extension_dir(&self, source_path: &Path, target_path: &Path){
        create_dir_all(target_path.clone()).expect("failed to create target directories");
        let mut contents_directory = source_path.to_path_buf();
        contents_directory.push("Contents");
        if contents_directory.exists() {
            self.copy_files(&contents_directory, target_path);
        }
    }

    fn copy_files(&self, source_path: &Path, target_path: &Path){
        let mut copy_options = CopyOptions::new();
        copy_options.content_only = true;
        copy_options.overwrite = true;
        copy_options.copy_inside = true;
        copy(source_path, target_path, &copy_options)
            .expect("failed to extract file, on windows this might be because the resulting output file path was too long");
    }

}

#[cfg(test)]
mod test{
    use super::*;
    use std::{fs::{File, read_to_string}, io::Write};
    use tempfile::tempdir;

    #[test]
    fn file_with_manifest_that_does_not_contain_extension_dir_copies_contents_folder_to_install_directory(){
        let source_path = tempdir().unwrap().path().to_path_buf();
        create_dir_all(source_path.clone()).unwrap();
        create_manifest_with_content(&source_path, "{}");
        let mut file_to_move = source_path.clone();
        file_to_move.push("Contents/Common7/somefile.txt");
        create_dir_all(file_to_move.parent().unwrap()).unwrap();
        let mut file = File::create(file_to_move).unwrap();
        file.write_all("successfully moved!!!".as_bytes()).unwrap();
        
        let target_path = tempdir().unwrap().path().to_path_buf();
        VsixExtractedFileMover::new().copy_vsix_contents_to_directory(&source_path, &target_path);
        let mut expected_file = target_path.clone();
        expected_file.push("Common7/somefile.txt");
        assert_eq!(read_to_string(expected_file).unwrap(), "successfully moved!!!");
    }

    #[test]
    fn file_with_manifest_that_does_not_contain_extension_dir_copies_nothing_if_contents_folder_does_not_exist(){
        let source_path = tempdir().unwrap().path().to_path_buf();
        create_dir_all(source_path.clone()).unwrap();
        create_manifest_with_content(&source_path, "{}");
        
        let target_path = tempdir().unwrap().path().to_path_buf();
        VsixExtractedFileMover::new().copy_vsix_contents_to_directory(&source_path, &target_path);
        assert_eq!(target_path.read_dir().into_iter().count(), 1);
    }

    #[test]
    fn file_with_manifest_that_contains_extension_dir_moves_most_content_to_directory_specified(){
        let source_path = tempdir().unwrap().path().to_path_buf();
        create_dir_all(source_path.clone()).unwrap();
        create_manifest_with_content(&source_path, "{\"extensionDir\": \"[installdir]\\\\nested_dir\"}");
        let mut file_to_move = source_path.clone();
        file_to_move.push("otherDirectory/somefile.txt");
        create_dir_all(file_to_move.parent().unwrap()).unwrap();
        let mut file = File::create(file_to_move).unwrap();
        file.write_all("another success!!!".as_bytes()).unwrap();
        
        let target_path = tempdir().unwrap().path().to_path_buf();
        VsixExtractedFileMover::new().copy_vsix_contents_to_directory(&source_path, &target_path);
        let mut expected_file = target_path.clone();
        expected_file.push("nested_dir/otherDirectory/somefile.txt");
        assert_eq!(read_to_string(expected_file).unwrap(), "another success!!!");
    }

    #[test]
    fn file_with_manifest_that_contains_bom(){
        let source_path = tempdir().unwrap().path().to_path_buf();
        create_dir_all(source_path.clone()).unwrap();
        create_manifest_with_content(&source_path, "\u{feff}{\"extensionDir\": \"[installdir]\\\\nested_dir\"}");
        let mut file_to_move = source_path.clone();
        file_to_move.push("otherDirectory/somefile.txt");
        create_dir_all(file_to_move.parent().unwrap()).unwrap();
        let mut file = File::create(file_to_move).unwrap();
        file.write_all("another success!!!".as_bytes()).unwrap();
        
        let target_path = tempdir().unwrap().path().to_path_buf();
        VsixExtractedFileMover::new().copy_vsix_contents_to_directory(&source_path, &target_path);
        let mut expected_file = target_path.clone();
        expected_file.push("nested_dir/otherDirectory/somefile.txt");
        assert_eq!(read_to_string(expected_file).unwrap(), "another success!!!");
    }

    fn create_manifest_with_content(parent_path: &Path, content: &str){
        let mut manifest_file = parent_path.to_path_buf();
        manifest_file.push("manifest.json");
        let mut file = File::create(manifest_file.clone()).expect(format!("{:?}", manifest_file.clone()).as_str());
        file.write_all(content.as_bytes()).unwrap();
    }


}