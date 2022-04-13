mod ast;

extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate core;

use std::fs;
use std::collections::HashMap;
use std::fmt::Display;

use pest::Parser;
use pest::iterators::Pairs;

#[derive(Parser)]
#[grammar = "tar-script.pest"]
pub struct TarParser;

fn log<I: Display>(inp: I) {
    println!("Log: '{}'", inp);
}

fn error<I: Display>(inp: I, call_stack: &Vec<String>) {
    println!("Error: {}", inp);
    println!("call_stack: {:?}", call_stack);
    panic!();
}


pub fn interpret(defs: Vec<ast::AstNode>, funcs: HashMap<String, ast::AstNode>) {
    let mut entry = String::new();
    if defs.len() > 0 {
        for d_i in defs {
            if let ast::AstNode::Definition{target, value} = d_i {
                if target == "entry" {
                    entry = value;
                }
            }
            else if let ast::AstNode::Import(value) = d_i{
                println!("TODO: import functions");
            }
            else {panic!("definition has to be 'definition' or 'import'.")}
        }

        if entry != "" {
            println!("Running entry function '{}'.", entry);

            let mut call_stack: Vec<String> = vec![];
            run_fn(entry, &funcs, &mut call_stack, vec![("int".to_owned(), ast::AstNode::Integer(10))]);
        }
        else {
            todo!("run lang as scripting lang")
        }
    }
    else {
        panic!("Why are you using the lang without imports or a main statement that makes literaly no sense")
    }
} 

// inputs is in the form of vec(type, value)
fn run_fn(name: String, funcs: &HashMap<String, ast::AstNode>, call_stack: &mut Vec<String>, inputs: Vec<(String, ast::AstNode)>) -> Option<(String, ast::AstNode)> {

    if call_stack.len() >= 100 {
        panic!("stack overflow error");
    }

    //TODO: implement user defined functions
    if name == "log".to_owned() {
        match &inputs[0].1 {

            ast::AstNode::Integer(int) => {
                log(int);
            }

            ast::AstNode::String(stri) => {
                log(stri);
            }

            _ => {
                panic!("unexpected input '{:?}'", &inputs[0].1);
            }
        }
        return None;
    }


    let func = funcs.get(&name).unwrap();
    if let ast::AstNode::FuncDef{ident, args, ret_ty, block}  = func {
        if !args.is_none() && args.clone().unwrap().len() != inputs.len() {
            error(format!("incorrect number of inputs to function: {}", ident), call_stack);
        }
        
        // k: ident v: type, value
        let mut vars: HashMap<String, (String, ast::AstNode)> = HashMap::new();

        if !args.is_none() {
            let tmp = args.clone().unwrap();
            let mut i = 0;
            for arg in tmp {
                if let ast::AstNode::Arg{ident, ty} = arg {
                    let inp = inputs[i].clone();
                    if inp.0 != ty {panic!("mismatched types expected '{}' but got '{}'", ty, inp.0)}
                    vars.insert(ident, (ty.clone(), get_val_checked(Box::new(inp.1), inp.0, funcs, call_stack, &vars)));
                }
                i+=1;
            }
        }

        for item in block {
            match item.clone() {
                ast::AstNode::Declaration {ty, ident, val} => {
                    let v = get_val_checked(val, ty.clone(), funcs, call_stack, &vars);
                    vars.insert(ident, (ty, v));
                }

                ast::AstNode::ValAssign {ident, val} => {
                    let (ty, _) = vars.get(&ident).unwrap();
                    let v = get_val_checked(val, ty.clone(), funcs, call_stack, &vars);
                    vars.insert(ident, (ty.clone(), v));
                }

                ast::AstNode::FuncCall {ident, args} => {
                    call_fn(ident, args, funcs, call_stack, &vars);
                }

                ast::AstNode::ReturnStmt(stmt) => {
                    let ret = ret_ty.clone().unwrap();

                    return Some((ret.clone(), get_val_checked(stmt, ret, funcs, call_stack, &vars)));
                }

                _ => {
                    panic!("invalid part of function block: '{:?}'", item);
                }
            }
        }
    }
    return None;
}

fn call_fn(ident: String, args: Vec<ast::AstNode>, funcs: &HashMap<String, ast::AstNode>, call_stack: &mut Vec<String>, vars: &HashMap<String, (String, ast::AstNode)>) -> Option<(String, ast::AstNode)> {
    call_stack.push(ident.clone());
    let mut ins = vec![];
    for arg in args {
        ins.push(get_val(Box::new(arg), funcs, call_stack, vars));
    }
    let res = run_fn(ident, funcs, call_stack, ins);
    call_stack.pop();
    return res;
}

fn eval_binop(op: ast::BinOp, lhs: Box<ast::AstNode>, rhs: Box<ast::AstNode>, funcs: &HashMap<String, ast::AstNode>, call_stack: &mut Vec<String>, vars: &HashMap<String, (String, ast::AstNode)> ) -> (String, ast::AstNode, ) {

    match op {
        ast::BinOp::Plus => {
            let l = get_int(lhs, funcs, call_stack, vars);
            let r = get_int(rhs, funcs, call_stack, vars);

            return ("int".to_owned(), ast::AstNode::Integer(l+r));
        }

        ast::BinOp::Minus => {
            let l = get_int(lhs, funcs, call_stack, vars);
            let r = get_int(rhs, funcs, call_stack, vars);

            return ("int".to_owned(), ast::AstNode::Integer(l-r));
        }

        ast::BinOp::Mul => {
            let l = get_int(lhs, funcs, call_stack, vars);
            let r = get_int(rhs, funcs, call_stack, vars);

            return ("int".to_owned(), ast::AstNode::Integer(l*r));
        }

        ast::BinOp::Div => {
            let l = get_int(lhs, funcs, call_stack, vars);
            let r = get_int(rhs, funcs, call_stack, vars);

            return ("int".to_owned(), ast::AstNode::Integer(l/r));
        }

        _ => {
            panic!("unsupported op: {:?}", op);
        }
    }
}

fn get_int(val: Box<ast::AstNode>, funcs: &HashMap<String, ast::AstNode>, call_stack: &mut Vec<String>, vars: &HashMap<String, (String, ast::AstNode)> ) -> i32 {
    if let ast::AstNode::Integer(int) = get_val_checked(val, "int".to_owned(), funcs, call_stack, vars) {
        return int;
    }
    error("expected int", call_stack);
    panic!();
}

fn get_val(val: Box<ast::AstNode>, funcs: &HashMap<String, ast::AstNode>, call_stack: &mut Vec<String>, vars: &HashMap<String, (String, ast::AstNode)> ) -> (String, ast::AstNode, ) {
    match *val {
        ast::AstNode::Integer(int) => {
            return ("int".to_owned(), ast::AstNode::Integer(int));
        }

        ast::AstNode::String(string) => {
            return ("string".to_owned(), ast::AstNode::String(string))
        }

        ast::AstNode::FuncCall{ident, args} => {
            return call_fn(ident, args, funcs, call_stack, vars).unwrap();
        }

        ast::AstNode::BinOp{op, lhs, rhs} => {
            return eval_binop(op, lhs, rhs, funcs, call_stack, vars);
        }

        ast::AstNode::Ident(ident) => {
            return vars.get(&ident).unwrap().clone();
        }

        _ => {
            panic!("unimplemented '{:?}'", val);
        }
    }
}

fn get_val_checked(val: Box<ast::AstNode>, ty: String, funcs: &HashMap<String, ast::AstNode>, call_stack: &mut Vec<String>, vars: &HashMap<String, (String, ast::AstNode)>) -> ast::AstNode {
    let (t, v) = get_val(val, funcs, call_stack, vars);

    if t==ty {return v;};

    panic!("Expected value '{}' but got '{}'", ty, t);
}

fn main() {
    let program = fs::read_to_string("C:/Users/wowwi/rust/engine/tar-script/src/main.tar").unwrap();

    let pairs: Pairs<Rule> = TarParser::parse(Rule::Program, program.as_str()).unwrap();

    let (defs, funcs) = ast::parse_to_ast(pairs);


    interpret(defs, funcs);
}

// TODO:  interpretation