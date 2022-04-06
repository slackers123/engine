/// # INFO TO FUTURE SLACKERS
/// remove my "unused" and "deadcode" tags when finishing, so you can make sure that you can still improve some stuff here and there

mod ast;

extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate core;

// use std::any::{Any, TypeId};
use std::fs;

use pest::Parser;
use pest::iterators::Pairs;

#[derive(Parser)]
#[grammar = "tar-script.pest"]
pub struct TarParser;

fn main() {
    let program = fs::read_to_string("./src/main.tar").unwrap();

    #[allow(unused_mut)]
    let mut pairs: Pairs<Rule> = TarParser::parse(Rule::Program, program.as_str()).unwrap();

    ast::parse_to_ast(pairs);
}

// TODO:  interpretation