// use anyhow::Result;
use wast::{WastDirective};
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
                println!("Assert\n  {:?}\n  {:?}\n  {:?}", span, exec, results);
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
                let s = &field.ty.inline;                // unable to reach params (Box or Option)
                // print!("(func ${:?} (param $a {:?}\n",field.exports.names[0], s);
                println!("  Function Exports: {:?}\n  Function Kind: {:?}\n  Function type: {:?}", field.exports, field.kind, field.ty);
                match &field.kind {
                    wast::FuncKind::Inline {locals, expression} => {
                        println!("{:?}",expression.instrs[0]);      // instrs cannot be indexed; how to determine what variant it is w/o match (500+)
                    }
                    _ => {}
                }
            },
            _ => {}
        }
    }

}

