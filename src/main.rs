use wast::{Import, Func, kw};
use wast::parser::{Parser, Parse, Result};
use wast::parser::{self, ParseBuffer};

struct OnlyImportsAndFunctions<'a> {
    imports: Vec<Import<'a>>,
    functions: Vec<Func<'a>>,
}

impl<'a> Parse<'a> for OnlyImportsAndFunctions<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        // While the second token is `import` (the first is `(`, so we care
        // about the second) we parse an `ast::ModuleImport` inside of
        // parentheses. The `parens` function here ensures that what we
        // parse inside of it is surrounded by `(` and `)`.
        let mut imports = Vec::new();
        while parser.peek2::<kw::import>() {
            let import = parser.parens(|p| p.parse())?;
            imports.push(import);
        }

        // Afterwards we assume everything else is a function. Note that
        // `parse` here is a generic function and type inference figures out
        // that we're parsing functions here and imports above.
        let mut functions = Vec::new();
        while !parser.is_empty() {
            let func = parser.parens(|p| p.parse())?;
            functions.push(func);
        }

        Ok(OnlyImportsAndFunctions { imports, functions })
    }
}

fn main() -> Result<()> {
    // take in the document name from terminal
    // let args = env::args().collect::<Vec<_>>();
    // if args.len() != 2 {
    //     println!("Usage: {} in.wasm", args[0]);
    //     return Ok(());
    // }

    let wat = "(module
        (func (export \"add\") (param $a i32) (param $b i32) (result i32) 
           (i32.add (local.get $a) (local.get $b)) 
       ) 
   ) 
   (assert_return (invoke \"add\" (i32.const 1) (i32.const 1)) (i32.const 2))";

    let buf = ParseBuffer::new(wat)?;
    let module = parser::parse::<wast::Wast>(&buf)?;

    println!("buf: {:?}", module.directives);

    // println!("buf: {}", module);

    // store the document values as a vector (buf) for the parser
    // let buf: Vec<u8> = std::fs::read(&args[1])?;

    // let val = parser::parse::<u32>(&buf)?;

    Ok(())
}

