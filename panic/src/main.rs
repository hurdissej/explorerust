use std::fs::File;
use std::io::ErrorKind;

fn main() {
    //panic!("Crash and burn");

    let file = File::open("doesnotexist.txt");

    match file {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("doesnotexist.txt"){
                Ok(fc) => fc,
                Err(e) => {
                    panic!("Failed when creating file {}", e)
                }
            }
        }, 
        Err(error) => {panic!("Problem opening file {}", error)}
    };

}
