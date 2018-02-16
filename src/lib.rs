use std::fs::File;
use std::io::prelude::*;
use std::io::Error;

pub fn concat(file_names: &Vec<&str>) -> Result<String, Error> {
    let mut file_contents: Vec<String> = Vec::new();

    for file_name in file_names {
        file_contents.push(get_file_content(file_name.to_string())?);
    }

    Ok(file_contents.join(""))
}

pub fn get_file_content(file_name: String) -> Result<String, Error> {
    match File::open(file_name) {
        Ok(mut file) => {
            let mut file_content = String::new();

            file.read_to_string(&mut file_content)?;

            Ok(file_content)
        },
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests;
