use pest::iterators::{Pair, Pairs};
use pest::Parser;

use std::collections::HashMap;

use crate::Rule;
use crate::TarParser;

#[derive(Debug, Clone)]
pub enum BinOp {// types of binary operations
    Plus,
    Minus,
    Mul,
    Div,
}

#[derive(Debug, Clone)]
pub enum BoolOp {// types of boolean operations
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanEqual,
    LessThanEqual,
}

#[derive(Debug, Clone)]
pub enum AstNode {
    Definition{
        target: String,
        value: String,
    },
    Import(Vec<String>), // The string vector is the "path" to import e.g. std::log -> ["std", "log"]
    FuncDef {
        ident: String, // name of the function
        args: Option<Vec<AstNode>>, // arguments the function requires (optional)
        ret_ty: Option<String>, // type to be returned (optional)
        block: Vec<AstNode>, // The function block (code inside the function)
    },
    Arg{
        ident: String, // name of the argument
        ty: String, // type of the argument
    },
    Ident(String),
    Integer(i32),
    Bool(bool),
    Float(f64),
    String(String),
    Array(Vec<AstNode>),
    BinOp { // binary operation (operation with two arguments)
        op: BinOp, // type of operation
        lhs: Box<AstNode>, // left side of operation
        rhs: Box<AstNode>, // right side of operation
    },
    BoolOp { // operation resulting in a boolean
        op: BoolOp,
        lhs: Box<AstNode>, // left side of operation
        rhs: Box<AstNode>, // right side of operation
    },
    WhileLoop {
        condition: Box<AstNode>,
        block: Vec<AstNode>,
    },
    IfStmt{
        condition: Box<AstNode>,
        block: Vec<AstNode>,
        else_if_stmt: Option<Vec<AstNode>>, // optional vector of ElseIf statements
        else_stmt: Option<Vec<AstNode>>, // optional additional block (block = Vec of AstNodes)
    },
    ElseIf {
        condition: Box<AstNode>,
        block: Vec<AstNode>,
    },
    ReturnStmt(Box<AstNode>), // Value is the expression to be evaluated for returning
    Declaration { // A variable declaration e.g. int a = 10;
        ty: String,
        ident: String,
        val: Box<AstNode>,
    },
    ValAssign { // a value assignment to a variable e.g. a = 20;
        ident: String,
        val: Box<AstNode>
    },
    FuncCall { // a call to a function e.g. log("Hello, World!");
        ident: String, // name of the function to be called
        args: Vec<AstNode>, // array of arguments given to the function (optional)
    },
    // Expr(Box<AstNode>), // Can either be a calculation or a string
}
pub fn is_const(st: AstNode) -> bool {
    match st {
        AstNode::Integer(_) => {true}
        AstNode::String(_) => {true}
        AstNode::Bool(_) => {true}
        _ => {false}
    }
}

pub fn parse_file_to_ast(file: String) -> (Vec<AstNode>, HashMap<String, AstNode>) {
    let parsed = TarParser::parse(Rule::Program, file.as_str()).unwrap();

    return parse_to_ast(parsed);
}


pub fn parse_to_ast(parsed: Pairs<Rule>) -> (Vec<AstNode>, HashMap<String, AstNode>){
    
    let mut defs = vec![];
    let mut funcs: HashMap<String, AstNode> = HashMap::new();

    for pair in parsed {
        match pair.as_rule() {
            Rule::definition => {
                defs.push(parse_def(pair));
            }
            Rule::import =>{
                defs.push(parse_import(pair));
            }
            Rule::functions => {
                for func in pair.into_inner() {
                    let name = get_ident(func.clone());
                    let fun = parse_fn(func, name.clone());

                    let test = funcs.insert(name.clone(), fun);
                    if !test.is_none() {panic!("The function {} already exists", name)};
                }
            }

            Rule::EOI => {break;}

            some => panic!("invalid top level input to ast parser: {:?}", some)
        }
    }

    return (defs, funcs);
}

fn parse_fn(func: Pair<Rule>, name: String) -> AstNode {
    let inner = func.into_inner();

    let ident = name;
    let mut args: Option<Vec<AstNode>> = None;
    let mut ret_ty: Option<String> = None;
    let mut block: Vec<AstNode> = vec![];

    for i in inner {
        match i.as_rule() {
            Rule::ident => {}
            Rule::args => {
                args = parse_fn_args(i);
            }
            Rule::retTy => {
                ret_ty = parse_fn_ret_ty(i);
            }
            Rule::block => {
                block = parse_fn_block(i);
            }
            _ => {panic!()}
        }
    }

    return AstNode::FuncDef {
        ident,
        args,
        ret_ty,
        block,
    };
}

fn parse_fn_block(func: Pair<Rule>) -> Vec<AstNode> {
    let mut block = vec![];

    let inner = func.into_inner();

    for j in inner {
        match j.as_rule() {
            Rule::funcCall => {
                block.push(parse_func_call(j));
            }

            Rule::returnStmt => {
                block.push(parse_ret_stmt(j));
            }
            Rule::decl => {
                block.push(parse_decl(j));
            }
            Rule::valAssign => {
                block.push(parse_val_assign(j));
            }

            Rule::ifStmt => {
                block.push(parse_if_stmt(j));
            }
            Rule::whileLoop => {
                block.push(parse_while_loop(j));
            }
            _ => {
                panic!("Unknown statemen: {:?}", j.as_rule());
            }
        }
    }
    return block;
}

fn parse_while_loop(lop: Pair<Rule>) -> AstNode {
    let mut inner = lop.into_inner();
    let condition = Box::new(parse_condition(inner.next().unwrap()));
    let block = parse_fn_block(inner.next().unwrap());

    return AstNode::WhileLoop {
        condition,
        block,
    }
}

fn parse_if_stmt(stmt: Pair<Rule>) -> AstNode {
    let mut inner = stmt.into_inner();
    let condition = Box::new(parse_condition(inner.next().unwrap()));
    let block = parse_fn_block(inner.next().unwrap());

    let mut else_if: Vec<AstNode> = vec![];

    let mut else_stmt: Option<Vec<AstNode>> = None;

    let mut cond: Option<Pair<Rule>> = None;
    for els in inner {
        match els.as_rule() {
            Rule::condition => {
                cond = Some(els);
            }
            Rule::block => {
                if cond.is_some() {
                    else_if.push(AstNode::ElseIf{
                        condition: Box::new(parse_condition(cond.clone().unwrap())),
                        block: parse_fn_block(els),
                    });
                    cond = None;
                }
                else {
                    else_stmt = Some(parse_fn_block(els));
                }
            }
            _ => {panic!("idk")}
        }
    }
    let mut else_if_stmt = None;
    if else_if.len() > 0 {
        else_if_stmt = Some(else_if);
    }
    
    return AstNode::IfStmt {
        condition,
        block,
        else_if_stmt,
        else_stmt,
    };
}

fn parse_condition(cond: Pair<Rule>) -> AstNode {
    let mut inner = cond.into_inner();
    let next = inner.next().unwrap();

    match next.as_rule() {
        Rule::boolExpr => {
            return parse_bool_expr(next);
        }
        Rule::bool => {
            return parse_bool(next);
        }
        Rule::innerFuncCall => {
            return parse_func_call(next);
        }
        _=> panic!("unknown")
    }
}

fn parse_bool_expr(expr: Pair<Rule>) -> AstNode {
    let mut inner = expr.into_inner();

    let op: BoolOp;

    let lhs = parse_expr(inner.next().unwrap());

    match inner.next().unwrap().as_rule() {
        Rule::equal => {
            op = BoolOp::Equal;
        }
        Rule::notEqual => {
            op = BoolOp::NotEqual;
        }
        Rule::greaterThan => {
            op = BoolOp::GreaterThan;
        }
        Rule::lessThan => {
            op = BoolOp::LessThan;
        }
        Rule::greaterThanEqual => {
            op = BoolOp::GreaterThanEqual;
        }
        Rule::lessThanEqual => {
            op = BoolOp::LessThanEqual;
        }

        some => {panic!("unexpected boolean operation '{:?}'", some)}
    }

    let rhs = parse_expr(inner.next().unwrap());

    return AstNode::BoolOp {
        op,
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
    }
}

fn parse_bool(val: Pair<Rule>) -> AstNode {
    if val.as_str() == "true" {
        return AstNode::Bool(true);
    }
    else if val.as_str() != "false" {
        panic!("wrong input to bool!");
    }
    return AstNode::Bool(false);
}

fn parse_val_assign(val_assign: Pair<Rule>) -> AstNode {
    let mut inner = val_assign.into_inner();
    let ident = inner.next().unwrap().as_str().to_owned();

    return AstNode::ValAssign {
        ident,
        val: Box::new(parse_expr(inner.next().unwrap())),
    }
}

fn parse_decl(decl: Pair<Rule>) -> AstNode {
    let mut inner = decl.into_inner();
    let ty = inner.next().unwrap().as_str().to_owned();
    let ident = inner.next().unwrap().as_str().to_owned();

    return AstNode::Declaration {
        ty,
        ident,
        val: Box::new(parse_expr(inner.next().unwrap())),
    };
}

fn parse_ret_stmt(stmt: Pair<Rule>) -> AstNode {
    return AstNode::ReturnStmt(Box::new(parse_expr(stmt.into_inner().next().unwrap())));
}

fn parse_func_call(call: Pair<Rule>) -> AstNode {

    let mut ident = String::new();
    let mut input = vec![];

    for p in call.into_inner() {
        match p.as_rule() {
            Rule::ident => {
                ident = p.as_str().to_owned();
            }

            Rule::Expr => {
                input.push(parse_expr(p));
            }
            _ => {
                panic!("Unknown part to funcCall: {:?}", p.as_rule());
            }
        }
    }
    
    
    return AstNode::FuncCall {
        ident,
        args: input,
    }
}

fn parse_expr(expr: Pair<Rule>) -> AstNode{

    match expr.as_rule() {
        Rule::Sum => {
            return parse_sum(expr);
        }
        Rule::string => {
            let str = expr.as_str();
            let str = &str[1..str.len() - 1];
            return AstNode::String(str.to_owned());
        }
        Rule::bool => {
            let str = expr.as_str();
            if str == "true" {
                return AstNode::Bool(true);
            }
            else if str == "false" {
                return AstNode::Bool(false);
            }
            else {panic!("something went seriously wrong")}
        }
        Rule::Expr => {
            return parse_expr(expr.into_inner().next().unwrap());
        }
        Rule::array => {
            return parse_array(expr);
        }

        _ => {
            panic!("Unknown rule in expression: {:?}", expr.as_rule())
        }
    }
}

fn parse_array(arr: Pair<Rule>) -> AstNode {
    let inner = arr.into_inner();

    let mut res = vec![];

    for i in inner {
        res.push(parse_expr(i));
    }

    return AstNode::Array(res);
}

fn parse_sum(sum: Pair<Rule>) -> AstNode {
    let mut inner = sum.into_inner();

    if inner.clone().count() == 1 {
        return parse_product(inner.next().unwrap());
    }
    else {
        let lhs = inner.next().unwrap();
        let op = inner.next().unwrap();
        let rhs = inner.next().unwrap();
        let ast_op: BinOp;

        match op.as_rule() {
            Rule::plus => {
                ast_op = BinOp::Plus;
            }
            Rule::minus => {
                ast_op = BinOp::Minus;
            }
            some => {
                panic!("invalid sum operation: {:?}", some);
            }
        }
        return AstNode::BinOp {
            op: ast_op,
            lhs: Box::new(parse_product(lhs)),
            rhs: Box::new(parse_product(rhs)),
        }
    }
}

fn parse_product(prod: Pair<Rule>) -> AstNode {
    let mut inner = prod.into_inner();

    if inner.clone().count() == 1 {
        return parse_value(inner.next().unwrap());
    }
    else {
        let lhs = inner.next().unwrap();
        let op = inner.next().unwrap();
        let rhs = inner.next().unwrap();
        let ast_op: BinOp;

        match op.as_rule() {
            Rule::div => {
                ast_op = BinOp::Div;
            }
            Rule::mul => {
                ast_op = BinOp::Mul;
            }
            some => {
                panic!("invalid sum operation: {:?}", some);
            }
        }
        return AstNode::BinOp {
            op: ast_op,
            lhs: Box::new(parse_value(lhs)),
            rhs: Box::new(parse_value(rhs)),
        }
    }
}

fn parse_value(val: Pair<Rule>) -> AstNode{

    match val.as_rule() {
        Rule::ident => {
            return AstNode::Ident(val.as_str().to_owned());
        }
        Rule::int => {
            return AstNode::Integer(val.as_str().parse().unwrap());
        }
        Rule::float => {
            let str = val.as_str();
            let str = &str[0..str.len() - 1];
            return AstNode::Float(str.parse().unwrap());
        }
        Rule::Expr => {
            return parse_expr(val);
        }
        Rule::innerFuncCall => {
            return parse_func_call(val);
        }

        _ => {
            panic!("Unknown value: {:?}", val.as_rule())
        }
    }
}

fn parse_fn_ret_ty(func: Pair<Rule>) -> Option<String> {
    return Some(func.as_str().to_owned());
}

fn parse_fn_args(func: Pair<Rule>) -> Option<Vec<AstNode>> {
    let mut args = vec![];

    let inner = func.into_inner();

    for arg in inner {
        args.push(AstNode::Arg{
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

fn parse_import(pair: Pair<Rule>) -> AstNode {
    let mut imports = vec![];

    for i in pair.into_inner() {
        imports.push(i.as_str().to_owned());
    }

    return AstNode::Import(imports);
}

fn parse_def(pair: Pair<Rule>) -> AstNode {
    let mut inner = pair.into_inner();

    return AstNode::Definition {
        target: inner.next().unwrap().as_str().to_owned(),
        value: inner.next().unwrap().as_str().to_owned(),
    }
}