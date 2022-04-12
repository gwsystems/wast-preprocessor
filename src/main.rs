// use anyhow::Result;
use anyhow::Result;
use wast::parser::{self, ParseBuffer};
use wast::ModuleField;
use wast::ModuleKind;
use wast::ValType;

use std::fs;
use std::path::Path;

fn main() -> Result<()> {

    let wat = r#"
        (module
            (func (export "add") (param $a i32) (param $b i32) (result i32)
                (i32.add (local.get $a) (local.get $b))
            )
        )
        (assert_return (invoke "add" (i32.const 1) (i32.const 1)) (i32.const 2))
    "#;

    let buf = ParseBuffer::new(wat)?;
    let ast = parser::parse::<wast::Wast>(&buf)?;

    for directive in ast.directives {
        match directive {
            wast::WastDirective::Module(mut _mod) => {
                if let ModuleKind::Text(txt) = &_mod.kind {
                    for field in txt {
                        if let ModuleField::Func(_func) = field {
                            let return_type = _func.ty.inline.clone().unwrap().results[0];
                            let c_return_type = match return_type {
                                ValType::I32 => "int",
                                ValType::I64 => "long int",
                                ValType::F32 => "float",
                                ValType::F64 => "double",
                                _ => panic!("AHHHH!"),
                            };
                            let mut c_param: Vec<String> = Vec::new();
                            for (_, _, p) in _func.ty.inline.clone().unwrap().params.iter() {
                                match p {
                                    ValType::I32 => c_param.push("int".to_string()),
                                    ValType::I64 => c_param.push("long int".to_string()),
                                    ValType::F32 => c_param.push("float".to_string()),
                                    ValType::F64 => c_param.push("double".to_string()),
                                    _ => panic!("AHHHH!"),
                                };
                            }
    
                            print!(
                                "IMPORT {} wasmf_{}({}",
                                c_return_type, _func.exports.names[0], c_param[0]
                            );
                            if c_param.len() > 1 {
                                for n in 1..c_param.len() {
                                    print!(
                                        ", {}",
                                        c_param[n]
                                    );
                                }
                            }
                            println!(");");
                        }
                    }
                    // And generate wasm module
                    let path: &Path = Path::new("test.wasm");
                    fs::write(path, _mod.encode().unwrap()).unwrap();
                }
            },

            wast::WastDirective::AssertReturn{span: _, exec, results} => {
                // println!("{:?}{:?}{:?}", span, exec, results);
                println!("\nint main(int argc, char* argv[]) {{");
                // println!("{:#?}", exec);
                match exec {
                    wast::WastExecute::Invoke(invoke) =>{
                        print!("\tawsm_assert(wasmf_{}(", invoke.name);
                        for p in invoke.args.iter() {
                            print!("{:?}",p.instrs[0]);
                        }
                        println!(" = {:?});", results[0]);
                    },
                    _ => {}
                }

                println!("}}");
            },

            _ => {}

        }
    }

    Ok(())
}

    // let vec = module.directives;

    // for i in vec.iter() {
    //     // println!("{:?}", i);
    //     match i {
    //         WastDirective::AssertReturn{span: _, exec, results: _} => {
    //             // println!("Assert\n  {:?}\n  {:?}\n  {:?}", span, exec, results);
    //             // let assert = wat.to_string().substring(span.offset, wat.len());
    //             // let new_assert = exec.invoke[0];
    //             match exec {
    //                 wast::WastExecute::Invoke(invoke) =>{
    //                     println!("{:?}", invoke.span.offset);
    //                 },
    //                 _ => {}
    //             }
    //         },
    //         WastDirective::Module(m) => {
    //             println!("Module");
    //             let content = &m.kind;
    //             match content {
    //                 wast::ModuleKind::Text(t) => {
    //                     extract_module(t);  // helper function
    //                 },
    //                 _ => {}
    //             }
    //         }
    //         _ => {}
    //     }
    // }

    // Ok(())


//  Helper Function to remove the function contents from the 
//      passed in module. Span, ID, and Name do not contain
//      helpful data in the case of the sample program.

// fn extract_module(text: &Vec<wast::ModuleField>) {
//     for t in text.iter() {
//         match t {
//             wast::ModuleField::Func(field) => {
//                 let _s = &field.ty.inline;                // unable to reach params (Box or Option)
//                 // print!("(func ${:?} (param $a {:?}\n",field.exports.names[0], s);
//                 println!("  Function Exports: {:?}\n  Function Kind: {:?}\n  Function type: {:?}", field.exports, field.kind, field.ty);
//                 match &field.kind {
//                     wast::FuncKind::Inline {locals: _, expression} => {
//                         println!("{:?}",expression.instrs[0]);      // instrs cannot be indexed; how to determine what variant it is w/o match (500+)
//                     }
//                     _ => {}
//                 }
//             },
//             _ => {}
//         }
//     }

// }

