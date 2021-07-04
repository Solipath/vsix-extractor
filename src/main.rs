use std::path::Path;

use tempfile::tempdir;
use vsix_extractor::{encoded_character_file_replacer::EncodedCharacterFileReplacer, file_decompressor::{FileDecompressor, FileDecompressorTrait}, vsix_extracted_file_mover::VsixExtractedFileMover};

pub mod recursive_file_looper;
pub mod vsix_extractor;

fn main() {
    let current_dir = std::env::current_dir().unwrap();
    let mut arguments = std::env::args();
    let mut source_dir = current_dir.clone();
    arguments.next();
    source_dir.push(arguments.next().unwrap());

    let mut target_dir = current_dir.clone();
    target_dir.push(arguments.next().unwrap());
    recursively_extract_all_vsix_files(&source_dir, &target_dir);
}

fn recursively_extract_all_vsix_files(source_dir: &Path, target_dir: &Path){
    recursive_file_looper::recursively_loop_through_files(source_dir, &|path|{
        if path.is_file() && path.extension().unwrap() == "vsix" {
            println!("{:?}", path.clone());
            let unzipped_file_location = tempdir().unwrap().into_path();
            FileDecompressor::new().decompress_file_to_directory(path, &unzipped_file_location);
            EncodedCharacterFileReplacer::new().recursively_replace_encoded_characters(&unzipped_file_location);
            VsixExtractedFileMover::new()
                .copy_vsix_contents_to_directory(&unzipped_file_location, target_dir);
        }
        path.to_path_buf()
    });
}

#[cfg(test)]
mod test{
    use std::{fs::read_to_string, path::PathBuf};

    use super::*;
    #[test] 
    fn extract_a_couple_vsix_files(){
        let mut source_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        source_file.push("tests/resources/multivsix");
        let target_path = tempdir().unwrap().path().to_path_buf();
        recursively_extract_all_vsix_files(&source_file, &target_path);
        let mut expected_file1 = target_path.clone();
        expected_file1.push("Common7/IDE/CommonExtensions/Microsoft/ManagedLanguages/VBCSharp/ExpressionEvaluators/catalog.json");
        let expected_content1 = r#"{"manifestVersion":"1.1","info":{"id":"Microsoft.CodeAnalysis.ExpressionEvaluator,version=3.10.0.2131811"},"packages":[{"id":"Microsoft.CodeAnalysis.ExpressionEvaluator","version":"3.10.0.2131811","type":"Vsix","payloads":[{"fileName":"ExpressionEvaluatorPackage.vsix","size":395218}],"vsixId":"21BAC26D-2935-4D0D-A282-AD647E2592B5","extensionDir":"[installdir]\\Common7\\IDE\\CommonExtensions\\Microsoft\\ManagedLanguages\\VBCSharp\\ExpressionEvaluators","installSizes":{"targetDrive":946696}}]}"#;
        assert_eq!(expected_content1, read_to_string(expected_file1).unwrap());
        let mut expected_file2 = target_path.clone();
        expected_file2.push("Team Tools/DiagnosticsHub/Collector/Agents/NativeMemoryCollectionAgent.dll");
        assert!(expected_file2.exists());
    }
}