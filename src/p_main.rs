// Convert .wast file to .wat
//      .wast file must first become .wasm file, the binary version
//      Use the following command to convert to .wasm file
//          wat2wasm [relative path to .wast] ==> wat2wasm src/add_test.wast 
//      Use the following command to run the parser program on the .wasm file
//          cargo run [relative path to .wasm] ==> cargo run add_test.wasm 

use anyhow::Result;
use std::env;
use wasmparser::{Parser, Payload};
// use wasmparser::FunctionSectionReader;

fn main() -> Result<()> {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        println!("Usage: {} in.wasm", args[0]);
        return Ok(());
    }

    let buf: Vec<u8> = std::fs::read(&args[1])?;
    for payload in Parser::new(0).parse_all(&buf) {
        match payload? {
            Payload::Version { .. } => {
                println!("====== Module");
            }
            Payload::ExportSection(s) => {
                for export in s {
                    let export = export?;
                    println!("  Export {} {:?}", export.field, export.kind);
                }
            }
            Payload::ImportSection(s) => {
                for import in s {
                    let import = import?;
                    println!("  Import {}::{}", import.module, import.field.unwrap());
                }
            }
            // FunctionSection
            Payload::FunctionSection(mut function_section_reader) => {
                for _ in 0..function_section_reader.get_count() {
                    let fn_type = function_section_reader.read().expect("function body");
                    println!("  Function of Type {}", fn_type);
                }
            }
            Payload::CodeSectionEntry (code_reader) => {
                for _ in 0..code_reader.range().end {
                    let mut binary_reader = code_reader.get_binary_reader();
                    while !binary_reader.eof() {
                        let pos = binary_reader.original_position();
                        let inst = binary_reader.read_operator()?;
                        println!("Pos: {} Inst: {:?}", pos, inst);
                    }
                    // assert!(binary_reader.read_var_u32().expect("local count") == 0);
                    let st = binary_reader.read_type().expect("code string");
                    println!("Code string: {:?}", st);
                }
                /*
                ====== Module
thread 'main' panicked at 'code string: BinaryReaderError { inner: BinaryReaderErrorInner { message: "Unexpected EOF", offset: 36, needed_hint: Some(27) } }', src/main.rs:49:58
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
                */
            }
            // result (not found in the parser)
            _other => {
                // println!("found payload {:?}", _other);
            }
        }
    }

    Ok(())
}

// use wasmparser::ExportSectionReader;

// fn main() {
//     println!("Entered");
//     let mut export_reader = ExportSectionReader::new(data, 0).unwrap();
//     for _ in 0..export_reader.get_count() {
//         let export = export_reader.read().expect("export");
//         println!("Export: {:?}", export);
//     }
// }