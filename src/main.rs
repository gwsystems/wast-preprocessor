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
            Payload::FunctionSection(s) => {
                /* 
                let mut function_reader = FunctionSectionReader::new(s, 0).unwrap();
                for _ in 0..function_reader.get_count() {
                    let ty = function_reader.read().expect("function type index");
                    println!("  Function {}", ty);
                }
                */
                let mut ty = s.read().expect("");
                /*
                cannot borrow `s` as mutable, as it is not declared as mutable

                cannot borrow as mutable
                */
                println!("  Function read {}", ty);
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