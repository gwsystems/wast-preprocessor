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

    let sp = module.directives;

    for i in sp.iter() {
        // println!("{:?}", i);
        match i {
            WastDirective::AssertReturn{span, exec, results} => {
                println!("{:?}", span);
                println!("{:?}", exec);
                println!("{:?}", results);
            },
            _ => {}
        }
    }

    Ok(())
}

