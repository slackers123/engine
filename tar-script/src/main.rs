/// # INFO TO FUTURE SLACKERS
/// remove my "unused" and "deadcode" tags when finishing, so you can make sure that you can still improve some stuff here and there

mod ast;

extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate core;

#[allow(unused)]
use std::any::{Any, TypeId};
use std::fs;
use std::collections::HashMap;

use pest::Parser;
use pest::iterators::{Pair, Pairs};

#[derive(Parser)]
#[grammar = "tar-script.pest"]
pub struct TarParser;


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
    let mut block = vec![];

    let inner = func.into_inner();

    for i in inner {
        if i.as_rule() == Rule::block {
            for j in i.into_inner() {
                match j.as_rule() {
                    Rule::funcCall => {
                        block.push(parse_funcCall(j));
                    }

                    Rule::returnStmt => {
                        // parse_ret_stmt(j);
                    }
                    #[allow(unused)]
                    some => {
                        panic!("Unknown statemen: {:?}", j.as_rule());
                    }
                }
            }
        }
    }
    return block;
}

#[allow(non_snake_case)]
fn parse_funcCall(call: Pair<Rule>) -> ast::AstNode {

    let mut ident = "".to_owned();
    let mut input = vec![];

    for p in call.into_inner() {
        match p.as_rule() {
            Rule::ident => {
                ident = p.as_str().to_owned();
            }

            Rule::Expr => {
                input.push(parse_expr(p));
            }

            #[allow(unused)]
            some => {
                panic!("Unknown part to funcCall: {:?}", p.as_rule());
            }
        }
    }
    
    
    return ast::AstNode::FuncCall {
        ident,
        args: input,
    }
}

fn parse_expr(expr: Pair<Rule>) -> ast::AstNode{

    match expr.as_rule() {
        Rule::Sum => {
            return parse_sum(expr);
        }
        Rule::string => {
            return ast::AstNode::String(expr.as_str().to_owned());
        }
        Rule::Expr => {
            return parse_expr(expr.into_inner().next().unwrap());
        }

        #[allow(unused)]
        some => {
            panic!("Unknown rule in expression: {:?}", expr.as_rule())
        }
    }
}

fn parse_sum(sum: Pair<Rule>) -> ast::AstNode {
    let mut inner = sum.into_inner();

    if inner.clone().count() == 1 {
        return parse_product(inner.next().unwrap());
    }
    else {
        let lhs = inner.next().unwrap();
        let op = inner.next().unwrap();
        let rhs = inner.next().unwrap();
        let ast_op: ast::BinOp;

        match op.as_rule() {
            Rule::plus => {
                ast_op = ast::BinOp::Plus;
            }
            Rule::minus => {
                ast_op = ast::BinOp::Minus;
            }
            some => {
                panic!("invalid sum operation: {:?}", some);
            }
        }
        return ast::AstNode::BinOp {
            op: ast_op,
            lhs: Box::new(parse_product(lhs)),
            rhs: Box::new(parse_product(rhs)),
        }
    }
}

fn parse_product(prod: Pair<Rule>) -> ast::AstNode {
    let mut inner = prod.into_inner();

    if inner.clone().count() == 1 {
        return parse_value(inner.next().unwrap());
    }
    else {
        let lhs = inner.next().unwrap();
        let op = inner.next().unwrap();
        let rhs = inner.next().unwrap();
        let ast_op: ast::BinOp;

        match op.as_rule() {
            Rule::div => {
                ast_op = ast::BinOp::Div;
            }
            Rule::mul => {
                ast_op = ast::BinOp::Mul;
            }
            some => {
                panic!("invalid sum operation: {:?}", some);
            }
        }
        return ast::AstNode::BinOp {
            op: ast_op,
            lhs: Box::new(parse_value(lhs)),
            rhs: Box::new(parse_value(rhs)),
        }
    }
}

fn parse_value(val: Pair<Rule>) -> ast::AstNode{

    match val.as_rule() {
        Rule::ident => {
            return ast::AstNode::Ident(val.as_str().to_owned());
        }
        Rule::int => {
            return ast::AstNode::Integer(val.as_str().parse().unwrap());
        }
        Rule::Expr => {
            return parse_expr(val);
        }
        Rule::innerFuncCall => {
            return parse_funcCall(val);
        }

        #[allow(unused)]
        some => {
            panic!("Unknown value: {:?}", val.as_rule())
        }
    }
}

fn parse_fn_ret_ty(func: Pair<Rule>) -> Option<String> {
    #[allow(unused_mut)]
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

    #[allow(unused_mut)]
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

    #[allow(unused_mut)]
    let mut pairs: Pairs<Rule> = TarParser::parse(Rule::Program, program.as_str()).unwrap();

    parse_to_ast(pairs);
}

// TODO:  interpretation