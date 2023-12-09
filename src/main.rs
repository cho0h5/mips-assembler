mod instruction;
mod parser;

use crate::instruction::Instruction;
use crate::parser::parse;

use std::path::Path;

fn write_binary_file(
    instructions: &[Box<dyn Instruction>],
    target_filename: &str,
) -> Result<(), std::io::Error> {
    use std::fs::File;
    use std::io::prelude::*;

    let path = Path::new(target_filename);

    let mut file = File::create(path)?;

    for instruction in instructions {
        let bin = instruction.convert();
        println!("{:08x}", bin);

        let buf: [u8; 4] = [
            (bin >> 24) as u8,
            (bin >> 16) as u8,
            (bin >> 8) as u8,
            bin as u8,
        ];

        file.write_all(&buf)?;
    }
    Ok(())
}

fn main() {
    let filename = std::env::args().nth(1).expect("no filename");

    println!("---- parse assembly file ----");
    let instructions = parse(&filename);
    for inst in &instructions {
        println!("{:?}", inst);
    }
    println!("-----------------------------\n");

    let mut target_filename = Path::new(&filename)
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    target_filename.push_str(".bin");

    println!("---- write binary file ----");
    if write_binary_file(&instructions, &target_filename).is_ok() {
        println!("---------------------------\n");
        println!("done");
    } else {
        println!("failed");
    };
}
