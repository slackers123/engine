mod ast;

extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate core;

use std::fs;
use std::collections::HashMap;

use pest::Parser;
use pest::iterators::Pairs;

#[derive(Parser)]
#[grammar = "tar-script.pest"]
pub struct TarParser;

pub enum Return {
    string(String),
    integer(i32),
}

pub fn interpret(defs: Vec<ast::AstNode>, funcs: HashMap<String, ast::AstNode>) {
    let mut entry = String::new();
    if defs.len() > 0 {
        let start = defs[0].clone();
        if let ast::AstNode::Definition{target, value} = start {
            if target == "entry" {
                entry = value;
            }
        }
        else {panic!("definition has to be ")}
    }
    else {
        panic!("Why are you using the lang without imports or a main statement that makes literaly no sense")
    }

    println!("{}", entry);
} 

fn run_fn(name: String, funcs: &HashMap<String, ast::AstNode>, call_stack: &Vec<String>) {

}

fn main() {
    let program = fs::read_to_string("C:/Users/wowwi/rust/engine/tar-script/src/main.tar").unwrap();

    let pairs: Pairs<Rule> = TarParser::parse(Rule::Program, program.as_str()).unwrap();

    let (defs, funcs) = ast::parse_to_ast(pairs);

    println!("{:?}", defs);
    println!("{:?}", funcs);

    // interpret(defs, funcs);
}

// TODO:  interpretation