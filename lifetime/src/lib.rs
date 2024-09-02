use std::fs::File;
use std::io::{self, Read};

pub trait FileProcessor {
    fn process(& mut self) -> io::Result<String>;
}

pub struct FileReader<'a>{
    // lifetime of the file reference to the struct, ensuring the reference 
    // is valid as long as the struct is used.
    pub file: &'a File,
}

impl<'a> FileProcessor for FileReader<'a> {
     fn process(& mut self) -> io::Result<String> {
        let mut content = String::new();
        self.file.read_to_string(&mut content)?;
        Ok(content)
    }
}
