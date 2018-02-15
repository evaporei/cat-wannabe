use std::fs::File;
use std::io::prelude::*;
use std::io::Error;

pub fn concat (file_names: Vec<String>) {

}

#[cfg(test)]
mod tests {
    #[test]
    fn get_file_content () {
        // TODO
        assert_eq!()
    }
}

pub fn get_file_content (file_name: String) -> Result<String, Error>{
    match File::open(file_name) {
        Ok(mut file) => {
            let mut file_content = String::new();

            file.read_to_string(&mut file_content)?;

            Ok(file_content)
        },
        Err(err) => Err(err),
    }
}
