extern crate failure;

use {
    failure::Error,
    std::{
        io::{
            Read,
            stdin,
        },
    },
};

fn main() -> Result<(), Error> {
    println!("\x1B[?1000h");
    let captured_output = {
        let mut input = String::new();
        let stdin = stdin();
        let mut stdin = stdin.lock();
        stdin.read_to_string(&mut input)?;
        input
    };
    println!("stdin: {:?}", captured_output);
    println!("\x1B[?1000l");
    Ok(())
}
