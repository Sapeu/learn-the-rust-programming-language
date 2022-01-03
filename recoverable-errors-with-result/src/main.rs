use std::fs::File;
use std::io;
use std::io::Read;
// use std::io::ErrorKind;
use std::error::Error;

// fn main() {
//     // let f: u32 = File::open("hello.txt"); let f = File::open("hello.txt"); let f = match f { Ok(file) => file,
//     //     Err(error) => match error.kind() {
//     //         ErrorKind::NotFound => match File::create("hello.txt") {
//     //             Ok(fc) => fc,
//     //             Err(e) => panic!("Problem creating the file: {:?}", e),
//     //         },
//     //         other_error => panic!("Problem opening the file: {:?}", other_error),
//     //     },
//     // };
// 
//     // let f = File::open("hello.txt").unwrap_or_else(|error| {
//     //     if error.kind() == ErrorKind::NotFound {
//     //         File::create("hello.txt").unwrap_or_else(|error| {
//     //             panic!("Problem creating the file: {:?}", error);
//     //         })
//     //     } else {
//     //         panic!("Problem opening the file: {:?}", error);
//     //     }
//     // });
//     // let f = File::open("hello.txt").expect("Failed to open hello.txt");
// 
//     // let f = read_username_from_file().expect("Failed to open hello.txt");
// 
//     let f = File::open("hello.txt")?;
//     println!("{:?}", f);
// }

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;
    Ok(())
}

fn read_username_from_file() -> Result<String, io::Error> {
    // fs::read_to_string("hello.txt")
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;

    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }

    Ok(s)
}
