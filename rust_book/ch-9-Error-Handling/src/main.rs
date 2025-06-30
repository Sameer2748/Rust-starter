use std::fs::File;
use std::io::{self, ErrorKind, Write}; // ✅ include Write here


fn main() {
     let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind(){
            ErrorKind::NotFound => match File::create("hello.txt"){
                  Ok(mut fc) => {
                    // Write some default content to the newly created file
                    match fc.write_all(b"Hello, this file was just created!") {
                        Ok(_) => println!("File created and written to successfully."),
                        Err(e) => panic!("File created but writing failed: {}", e),
                    }
                    fc // return the file
                }
                Err(e) => panic!("Problem opening the file: {}", e)
            },
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        }
    };
        println!("File opened or created successfully: {:?}", greeting_file);

}