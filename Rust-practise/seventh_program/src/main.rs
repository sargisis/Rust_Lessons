#![warn(clippy::all , clippy::pedantic)]

use std::env::current_dir;
use std::fmt::Error;
use std::fs::create_dir_all;
use std::path::PathBuf;
// use std::io::ErrorKind;

use std::io::Error as IOERROR;

fn main() -> std::io::Result<()> {
    // let current_path = current_dir();

   
    let mut target_path  = get_current_path()?;

    target_path.push("aaaaa");
    // match create_dir_all(&target_path) {
    //     Ok(()) => println!("Created: {target_path:?}"),
    //     Err(e) => match e.kind() {
    //         ErrorKind::InvalidFilename => {
                
    //         },
    //         unknown_e => {
    //             panic!("Error: {unknown_e:?}")
    //         }
    //     }
    // };

    create_dir_in(&target_path);

    println!("Created: {:?}" , target_path);
    Ok(()) 
}

fn get_current_path() -> Result<PathBuf , IOERROR >{
    let path = current_dir()?;

    Ok(path)
}

// fn create_dir_in(target: &PathBuf) -> Result<String ,IOERROR> { 
//     match create_dir_all(target) {
//         Ok(_) => Ok(format!("{}", target.to_string_lossy())),
//         Err(e) => Err(e),
//     }
// }

fn create_dir_in(target: &PathBuf) -> Result<(), IOERROR>{
    create_dir_all(target)?;

    Ok(())
}