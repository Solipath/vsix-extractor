use std::{fs::File, io::BufReader, path::Path};

#[cfg(test)]
use mockall::automock;
use zip::ZipArchive;

#[cfg_attr(test, automock)]
pub trait FileDecompressorTrait{
    fn decompress_file_to_directory(&self, file_path: &Path, target_path: &Path);
}

pub struct FileDecompressor;

impl FileDecompressor{
    pub fn new()-> Self{
        Self{}
    }
}

impl FileDecompressorTrait for FileDecompressor{
    fn decompress_file_to_directory(&self, file_path: &Path, target_path: &Path) {
        let zip_file = File::open(file_path).expect("failed to open file");
        let buffered_reader = BufReader::new(zip_file);
        ZipArchive::new(buffered_reader)
            .expect("failed to open vsix file")
            .extract(target_path)
            .expect("failed to extract file");
    }
}


#[cfg(test)]
mod test{
    use super::*;
    use std::{fs::{self}, path::PathBuf};
    use tempfile::tempdir;

    #[test]
    fn can_decompress_vsix_file() {
        let temp_dir = tempdir().unwrap();
        let mut target_directory = temp_dir.path().to_path_buf();
        target_directory.push("additional_node");
        let mut expected_destination_file = target_directory.clone();
        expected_destination_file.push("file_in_zip.txt");
        let mut source_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        source_file.push("tests/resources/zip_file.vsix");
        let file_decompressor = FileDecompressor::new();
        file_decompressor.decompress_file_to_directory(&source_file, &target_directory);
        
        let file_contents = fs::read_to_string(expected_destination_file.to_str().unwrap())
            .expect("something went wrong trying to read file");
        assert_eq!(file_contents, "this file is inside a zip file\n");
    }
}