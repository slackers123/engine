mod ast;

extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate core;

use std::any::{Any, TypeId};
use std::fs;
use pest::Parser;
use pest::iterators::{Pair, Pairs};

#[derive(Parser)]
#[grammar = "tar-script.pest"]
pub struct TarParser;

#[derive(Debug)]
pub enum VarType {
    Int,
    String,
    Function,
}

#[derive(Debug)]
pub struct Var {
    ty: VarType,
    val: Box<dyn Any>,
}

fn get_var(str: Pair<Rule>) -> Var {
    match str.as_rule() {
        Rule::int => {
            return Var {ty: VarType::Int,val: Box::new(10)}
        }
        _=>(panic!())
    }
}

fn interpret() {

}

fn main() {
    let program = fs::read_to_string("./src/main.tar").unwrap();

    let mut pairs: Pairs<Rule> = TarParser::parse(Rule::Program, program.as_str()).unwrap();

    println!("{:?}", pairs.next().unwrap());
}

// TODO: implement ast tree and interpretation