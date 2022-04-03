mod ast;

extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate core;

use std::any::{Any, TypeId};
use std::fs;
use std::collections::HashMap;

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

fn parse_to_ast(parsed: Pairs<Rule>) -> (Vec<ast::AstNode>, HashMap<String, ast::AstNode>){
    
    let mut defs = vec![];
    let mut ast: HashMap<String, ast::AstNode> = HashMap::new();

    for pair in parsed {
        match pair.as_rule() {
            Rule::definitions => {
                defs.push(evaluate_definition(pair));
            }


            some => panic!("invalid top level input to ast parser: {:?}", some)
        }
    }

    return (defs, ast);
}

fn evaluate_definition(pair: Pair<Rule>) -> ast::AstNode {
    let mut def_nodes = vec![];

    for p in pair.into_inner() {
        match p.as_rule() {
            Rule::definition => {
                for t_v in p.into_inner() {
                    match t_v.as_rule() {
                        _=> {println!("t_v: {:?}", t_v);}
                    }
                }
                // todo!("seperate the two identifiers and push them to the def_nodes vec as a Definition");
                // def_nodes.push(ast::AstNode::Definition{
                //     target: p.clone().into_inner().next().unwrap().as_str().to_string(),
                //     value: p.clone().into_inner().next().unwrap().as_str().to_string()
                // });
            }
            Rule::import => {
                let mut idents = vec![];
                for strs in p.into_inner() {
                    idents.push(strs.as_str().to_owned());
                }
                def_nodes.push(ast::AstNode::Import(idents));
            }
            _ =>  {
                panic!("Unexpected input for definitions: {:?}", p);
            }
        }
    }

    let defs = ast::AstNode::Definitions(def_nodes);

    println!("defs: {:?}", defs);

    return defs;
}


fn main() {
    let program = fs::read_to_string("./src/main.tar").unwrap();

    let mut pairs: Pairs<Rule> = TarParser::parse(Rule::Program, program.as_str()).unwrap();

    parse_to_ast(pairs);
}

// TODO: implement ast tree and interpretation