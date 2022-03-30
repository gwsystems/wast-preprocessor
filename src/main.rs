use anyhow::Result;
// use rayon::prelude::*;
// use std::env;
use wast::Wat;
use wast::parser::{self, ParseBuffer};

fn main() -> Result<()> {
    // take in the document name from terminal
    // let args = env::args().collect::<Vec<_>>();
    // if args.len() != 2 {
    //     println!("Usage: {} in.wasm", args[0]);
    //     return Ok(());
    // }

    let wat = "(module \n\
        (func (export \"add\") (param $a i32) (param $b i32) (result i32) \n\
           (i32.add (local.get $a) (local.get $b)) \n\
       ) \n\
   ) \n\
   (assert_return (invoke \"add\" (i32.const 1) (i32.const 1)) (i32.const 2))";

    let buf = ParseBuffer::new(wat)?;
    let module = parser::parse::<Wat>(&buf)?;

    // println!("buf: {}", module);

    // store the document values as a vector (buf) for the parser
    // let buf: Vec<u8> = std::fs::read(&args[1])?;

    // let val = parser::parse::<u32>(&buf)?;

    Ok(())
}