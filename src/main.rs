use anyhow::Result;
use wast::parser::{self, ParseBuffer};
use wast::ModuleField;
use wast::ModuleKind;
use wast::ValType;

use std::env;
use std::fs;
use std::io::{Read, Write};
use std::path::Path;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let pwd = env::current_dir().unwrap();
    let mut dest = Path::new(&pwd);

    if args.len() == 3 {
        dest = Path::new(&args[2]);
    } else if args.len() != 2 {
        println!("Usage: {} in.wasm", args[0]);
        return Ok(());
    }

    let mut wast_file = fs::File::open(&args[1])?;
    let mut contents = String::new();
    wast_file.read_to_string(&mut contents)?;

    let mut d_num = 0;

    let buf = ParseBuffer::new(&contents)?;
    let ast = parser::parse::<wast::Wast>(&buf)?;

    let mut imports: Vec<String> = Vec::new();
    let mut functions: Vec<String> = Vec::new();

    for directive in ast.directives {
        match directive {
            wast::WastDirective::Module(mut _mod) => {
                if !imports.is_empty() {
                    write_to_file(d_num, &mut imports, &mut functions, &args[1], dest);
                    d_num += 1;
                }
                if let ModuleKind::Text(txt) = &_mod.kind {
                    for field in txt {
                        if let ModuleField::Func(_func) = field {
                            let c_return_type;

                            if _func.ty.inline.is_none()
                                || _func.ty.inline.clone().unwrap().results.len() == 0
                            {
                                c_return_type = "void";
                            } else {
                                let return_type = _func.ty.inline.clone().unwrap().results[0];
                                c_return_type = match return_type {
                                    ValType::I32 => "int",
                                    ValType::I64 => "int64_t",
                                    ValType::F32 => "float",
                                    ValType::F64 => "double",
                                    ValType::Ref(_) => panic!(
                                        "wast_preprocessor does not support reference types!"
                                    ),
                                    _ => panic!("{:?} is an invalid type!", return_type),
                                };
                            }
                            let mut c_param: Vec<String> = Vec::new();
                            if !_func.ty.inline.is_none() {
                                for (_, _, p) in _func.ty.inline.clone().unwrap().params.iter() {
                                    match p {
                                        ValType::I32 => c_param.push("int".to_string()),
                                        ValType::I64 => c_param.push("int64_t".to_string()),
                                        ValType::F32 => c_param.push("float".to_string()),
                                        ValType::F64 => c_param.push("double".to_string()),
                                        _ => panic!("AHHHH!"),
                                    };
                                }
                            }

                            if _func.exports.names.len() > 0 {
                                let mut line = format!(
                                    "extern {} wasmf_{}(",
                                    c_return_type, _func.exports.names[0]
                                );
                                if c_param.len() > 0 {
                                    // TODO: Cleanup string join logic
                                    line.push_str(&c_param[0]);

                                    for n in 1..c_param.len() {
                                        let s = format!(", {}", c_param[n]);
                                        line.push_str(&s);
                                    }
                                }
                                line.push_str(");");
                                // Push statement onto vector
                                imports.push(line.to_string());
                            }
                        }
                    }
                    // And generate wasm module
                    write_wasm(d_num, &mut _mod, &args[1], dest);
                }
            }

            wast::WastDirective::AssertReturn {
                span: _,
                exec,
                results,
            } => {
                // Skip assert_returns that don't return a value
                // TODO: Just execute these functions...
                if results.len() == 0 {
                    continue;
                };

                let mut line;
                match exec {
                    wast::WastExecute::Invoke(invoke) => {
                        line = format!("\tassert(wasmf_{}(", invoke.name);
                        let mut ct = 0;
                        // obtain arguments
                        for p in invoke.args.iter() {
                            match &p.instrs[0] {
                                wast::Instruction::I32Const(val) => {
                                    if ct > 0 {
                                        line.push_str(", ");
                                    }
                                    let s = format!("{:?}", val);
                                    line.push_str(&s);
                                    ct = ct + 1;
                                }
                                wast::Instruction::I64Const(val) => {
                                    if ct > 0 {
                                        line.push_str(", ");
                                    }
                                    let s = format!("{:?});", val);
                                    line.push_str(&s);
                                    ct = ct + 1;
                                }
                                wast::Instruction::F32Const(val) => {
                                    if ct > 0 {
                                        line.push_str(", ");
                                    }
                                    let s = format!("{:?}", val.bits as f32);
                                    line.push_str(&s);
                                    ct = ct + 1;
                                }
                                wast::Instruction::F64Const(val) => {
                                    if ct > 0 {
                                        line.push_str(", ");
                                    }
                                    let s = format!("{:?}", val.bits as f64);
                                    line.push_str(&s);
                                    ct = ct + 1;
                                }
                                _ => {
                                    panic!("Unknown type: {:?}", &p.instrs[0]);
                                }
                            }
                        }
                        // obtain results
                        line.push_str(") == ");
                        match &results[0] {
                            wast::AssertExpression::I32(val) => {
                                let s = format!("{:?});", val);
                                line.push_str(&s);
                            }
                            wast::AssertExpression::I64(val) => {
                                let s = format!("{:?});", val);
                                line.push_str(&s);
                            }
                            wast::AssertExpression::F32(val) => match &val {
                                wast::NanPattern::Value(v) => {
                                    let s = format!("(float){:?});", v.bits);
                                    line.push_str(&s);
                                }
                                wast::NanPattern::ArithmeticNan => {
                                    let s = format!("(float)NAN);");
                                    line.push_str(&s);
                                }
                                wast::NanPattern::CanonicalNan => {
                                    let s = format!("(float)NAN);");
                                    line.push_str(&s);
                                }
                            },
                            wast::AssertExpression::F64(val) => match &val {
                                wast::NanPattern::Value(v) => {
                                    let s = format!("(double){:?});", v.bits);
                                    line.push_str(&s);
                                }
                                wast::NanPattern::ArithmeticNan => {
                                    let s = format!("(double)NAN);");
                                    line.push_str(&s);
                                }
                                wast::NanPattern::CanonicalNan => {
                                    let s = format!("(double)NAN);");
                                    line.push_str(&s);
                                }
                            },
                            _ => {
                                panic!("Other Result");
                            }
                        }
                    }
                    _ => {
                        panic!("Other Exec");
                    }
                }
                // Push statement onto vector
                functions.push(line.to_string());
            }
            _ => { // skip the others
            }
        }
    }

    if !imports.is_empty() {
        write_to_file(d_num, &mut imports, &mut functions, &args[1], dest);
    }

    Ok(())
}

fn write_to_file(
    d_num: i32,
    imports: &mut Vec<String>,
    functions: &mut Vec<String>,
    file: &String,
    dest: &Path,
) {
    // create new file, set output
    let file_name = Path::new(file).file_stem().unwrap();
    println!("file_name: {:?}", file_name);

    let path = format!(
        "{}/{}_{}.c",
        dest.to_str().unwrap(),
        file_name.to_str().unwrap(),
        d_num
    );

    println!("Path: {}", path);

    let mut output = fs::File::create(path).unwrap();

    // print output to file
    writeln!(
        output,
        "#include <stdint.h>\n#include <assert.h>\n\n#include \"runtime.h\""
    )
    .expect("include statement");
    // imports
    for s in imports.iter() {
        writeln!(output, "{}", s).expect("import");
    }

    // main function
    writeln!(output, "\nint main(int argc, char* argv[]) {{").expect("main function declaration");

    writeln!(output, "\truntime_init();\n").expect("runtime_init");

    // function calls
    for f in functions.iter() {
        writeln!(output, "{}", f).expect("function");
    }

    writeln!(output, "}}").expect("");

    // clear vectors
    *imports = Vec::new();
    *functions = Vec::new();
}

fn write_wasm(d_num: i32, module: &mut wast::Module, file: &String, dest: &Path) {
    // create new file, set output
    let file_name = Path::new(file).file_stem().unwrap();
    println!("file_name: {:?}", file_name);

    let path = format!(
        "{}/{}_{}.wasm",
        dest.to_str().unwrap(),
        file_name.to_str().unwrap(),
        d_num
    );

    println!("Path: {}", path);

    // And generate wasm module
    let path: &Path = Path::new(&path);
    fs::write(path, module.encode().unwrap()).unwrap();
}
