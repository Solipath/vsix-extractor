use serde::Deserialize;


#[derive(Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Manifest{
    #[serde(rename="extensionDir")]
    extension_dir: Option<String>
}

impl Manifest {
    pub fn new(extension_dir: Option<String>)-> Self{
        Self{extension_dir}
    }
    pub fn get_extension_dir(&self)->Option<String>{
        return self.extension_dir.clone()
    }
}

#[cfg(test)]
mod test{

    use super::*;

    #[test]
    fn deserialize_manifest(){
        let manifest: Manifest = serde_json::from_str(r#"{"extensionDir": "extension\\dir"}"#).unwrap();
        assert_eq!(manifest, Manifest::new(Some("extension\\dir".to_string())));
    }
}