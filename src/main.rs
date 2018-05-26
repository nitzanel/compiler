#![feature(get_type_id)]
#![feature(box_syntax)]
#[macro_use]
extern crate serde_derive;
extern crate docopt;
use docopt::Docopt;
mod compiler;
use compiler::driver;

const USAGE: &'static str = "
Kalaidoscope compiler.

Usage: 
    compiler [options]

Options:
    -l, --lexer  Run only lexer and show its output.
    -p, --parser  Run till parser and show its output.
    -a, --assembly  Run till ASM builder and show its output.
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_lexer: bool,
    flag_parser: bool,
    flag_assembly: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let stage = if args.flag_lexer {
        driver::Stage::Tokens
    } else if args.flag_parser {
        driver::Stage::AST
    } else {
        driver::Stage::Assembly
    };

    compiler::driver::driver_loop(stage);
}
