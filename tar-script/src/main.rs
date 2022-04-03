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
    let mut funcs: HashMap<String, ast::AstNode> = HashMap::new();

    for pair in parsed {
        match pair.as_rule() {
            Rule::definitions => {
                defs.push(evaluate_definition(pair));
            }
            Rule::functions => {
                for func in pair.into_inner() {
                    let name = get_ident(func.clone());
                    let fun = parse_fn(func, name.clone());

                    funcs.insert(name, fun);
                }
            }

            Rule::EOI => {break;}

            some => panic!("invalid top level input to ast parser: {:?}", some)
        }
    }

    println!("{:?}", funcs);

    return (defs, funcs);
}

fn parse_fn(func: Pair<Rule>, name: String) -> ast::AstNode {

    return ast::AstNode::FuncDef {
        ident: name,
        args: parse_fn_args(func.clone()),
        ret_ty: parse_fn_ret_ty(func.clone()),
        block: parse_fn_block(func),
    };
}

fn parse_fn_block(func: Pair<Rule>) -> Vec<ast::AstNode> {

    return vec![];
}


fn parse_fn_ret_ty(func: Pair<Rule>) -> Option<String> {
    let mut fn_inner = func.into_inner();
    for inner in fn_inner {
        if inner.as_rule() == Rule::retTy {
            return Some(inner.as_str().to_owned())
        }
    }

    return None;
}

fn parse_fn_args(func: Pair<Rule>) -> Option<Vec<ast::AstNode>> {
    let mut args = vec![];

    let mut fun = func.into_inner();
    fun.next();

    let mut fun_inner = fun.next().unwrap().into_inner();

    if fun_inner.clone().next().unwrap().as_rule() != Rule::arg {
        return None;
    }

    for arg in fun_inner {
        args.push(ast::AstNode::Arg{
            ident: get_ident(arg.clone()),
            ty: get_ty(arg),
        });
    }

    if args.len() == 0 {return None};

    return Some(args);
}

fn get_ty(arg: Pair<Rule>) -> String {
    let mut inner = arg.into_inner();
    inner.next();
    return inner.next().unwrap().as_str().to_owned();
}

fn get_ident(func: Pair<Rule>) -> String {
    return func.into_inner().next().unwrap().as_str().to_owned();
}

fn evaluate_definition(pair: Pair<Rule>) -> ast::AstNode {
    let mut def_nodes = vec![];

    for p in pair.into_inner() {
        match p.as_rule() {
            Rule::definition => {
                let mut v = vec![];
                for t_v in p.into_inner() {
                    v.push(t_v.as_str().to_owned())
                }
                def_nodes.push(ast::AstNode::Definition{ target: v.pop().unwrap(), value: v.pop().unwrap()});
                if v.len() > 0 {
                    panic!("to many entries in definition vector");
                }
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

    return defs;
}


fn main() {
    let program = fs::read_to_string("./src/main.tar").unwrap();

    let mut pairs: Pairs<Rule> = TarParser::parse(Rule::Program, program.as_str()).unwrap();

    parse_to_ast(pairs);
}

// TODO: implement the code block parsing into ast tree and interpretation