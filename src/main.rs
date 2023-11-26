mod instruction;
mod parser;

use crate::instruction::Instruction;

use crate::parser::parse;

fn write_binary_file(instructions: &[Box<dyn Instruction>]) -> Result<(), std::io::Error> {
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;

    let path = Path::new("example.bin");

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

    println!("---- write binary file ----");
    if write_binary_file(&instructions).is_ok() {
        println!("---------------------------\n");
        println!("done");
    } else {
        println!("failed");
    };
}
