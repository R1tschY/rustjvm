use std::collections::HashMap;

use crate::exception::JResult;
use crate::LoadedClass;
use classfile::parse::parse_class_file;
use std::fs;
use std::fs::File;
use std::path::PathBuf;

pub struct ClassLoader {
    classes: HashMap<String, LoadedClass>,
    class_path: Vec<PathBuf>,
}

impl ClassLoader {
    pub fn load_class(&self, name: &str) -> JResult<LoadedClass> {
        let class_file = name.replace('.', "/");
        for class_path_entry in &self.class_path {
            let path = class_path_entry.join(&class_file);
            let mut file = File::open(path).unwrap();
            let class_file = parse_class_file(&mut file).unwrap();
        }

        unimplemented!()
    }
}
