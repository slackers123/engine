mod ast;
mod astint;
mod bcvm;

extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate core;

use pest::Parser;
use pest::iterators::Pairs;

use std::fs;

#[derive(Parser)]
#[grammar = "tar-script.pest"]
pub struct TarParser;

fn main() {
    let program = fs::read_to_string("C:/Users/wowwi/rust/engine/tar-script/src/main.tar").unwrap();

    let pairs: Pairs<Rule> = TarParser::parse(Rule::Program, program.as_str()).unwrap();

    let (defs, funcs) = ast::parse_to_ast(pairs);

    let program = fs::read_to_string("tar.lock").unwrap();

    bcvm::run_from_string(program).unwrap();
}

// TODO: Parse ast -> bytecode