// use anyhow::Result;
use wast::{ WastDirective};
use wast::parser::{self, ParseBuffer, Result};


fn main() -> Result<()> {

    let wat = r#"
        (module
            (func (export "add") (param $a i32) (param $b i32) (result i32)
                (i32.add (local.get $a) (local.get $b))
            )
        )
        (assert_return (invoke "add" (i32.const 1) (i32.const 1)) (i32.const 2))
    "#;

    let buff = ParseBuffer::new(wat)?;
    let module = parser::parse::<wast::Wast>(&buff)?;

    let vec = module.directives;

    for i in vec.iter() {
        // println!("{:?}", i);
        match i {
            WastDirective::AssertReturn{span, exec, results} => {
                println!("Assert");
                println!("  {:?}", span);
                println!("  {:?}", exec);
                println!("  {:?}", results);
                println!();
                println!();
            },
            WastDirective::Module(m) => {
                println!("Module");
                let content = &m.kind;
                match content {
                    wast::ModuleKind::Text(t) => {
                        extract_module(t);  // helper function
                    },
                    _ => {}
                }
            }
            _ => {}
        }
    }

    Ok(())
}

//  Helper Function to remove the function contents from the 
//      passed in module. Span, ID, and Name do not contain
//      helpful data in the case of the sample program.

fn extract_module(text: &Vec<wast::ModuleField>) {
    for t in text.iter() {
        match t {
            wast::ModuleField::Func(field) => {
                println!("  Function Exports: {:?}", field.exports);
                println!("  Function Kind: {:?}", field.kind);
                println!("  Function type: {:?}", field.ty);
            },
            _ => {}
        }
    }

}

