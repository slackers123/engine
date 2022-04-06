use pest::iterators::{Pair, Pairs};
use std::collections::HashMap;

use crate::Rule;

#[derive(Debug)]
pub enum BinOp {// types of binary operation
    Plus,
    Minus,
    Mul,
    Div,
}

#[derive(Debug)]
pub enum AstNode {
    Definitions(Vec<AstNode>), // The value is the array of type Definition or Import
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
    Float(f32),
    String(String),
    BinOp { // binary operation (operation with two arguments)
        op: BinOp, // type of operation
        lhs: Box<AstNode>, // left side of operation
        rhs: Box<AstNode>, // right side of operation
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
    #[allow(dead_code)]
    Expr(Box<AstNode>), // Can either be a calculation or a string
}



pub fn parse_to_ast(parsed: Pairs<Rule>) -> (Vec<AstNode>, HashMap<String, AstNode>){
    
    let mut defs = vec![];
    let mut funcs: HashMap<String, AstNode> = HashMap::new();

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

fn parse_fn(func: Pair<Rule>, name: String) -> AstNode {

    return AstNode::FuncDef {
        ident: name,
        args: parse_fn_args(func.clone()),
        ret_ty: parse_fn_ret_ty(func.clone()),
        block: parse_fn_block(func),
    };
}

fn parse_fn_block(func: Pair<Rule>) -> Vec<AstNode> {
    let mut block = vec![];

    let inner = func.into_inner();

    for i in inner {
        if i.as_rule() == Rule::block {
            for j in i.into_inner() {
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
                    _ => {
                        panic!("Unknown statemen: {:?}", j.as_rule());
                    }
                }
            }
        }
    }
    return block;
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
            return AstNode::String(expr.as_str().to_owned());
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
        Rule::Expr => {
            return parse_expr(val);
        }
        Rule::innerFuncCall => {
            return parse_func_call(val);
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

fn parse_fn_args(func: Pair<Rule>) -> Option<Vec<AstNode>> {
    let mut args = vec![];

    let mut fun = func.into_inner();
    fun.next();

    #[allow(unused_mut)]
    let mut fun_inner = fun.next().unwrap().into_inner();

    if fun_inner.clone().next().unwrap().as_rule() != Rule::arg {
        return None;
    }

    for arg in fun_inner {
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

fn evaluate_definition(pair: Pair<Rule>) -> AstNode {
    let mut def_nodes = vec![];

    for p in pair.into_inner() {
        match p.as_rule() {
            Rule::definition => {
                let mut v = vec![];
                for t_v in p.into_inner() {
                    v.push(t_v.as_str().to_owned())
                }
                def_nodes.push(AstNode::Definition{ target: v.pop().unwrap(), value: v.pop().unwrap()});
                if v.len() > 0 {
                    panic!("to many entries in definition vector");
                }
            }
            Rule::import => {
                let mut idents = vec![];
                for strs in p.into_inner() {
                    idents.push(strs.as_str().to_owned());
                }
                def_nodes.push(AstNode::Import(idents));
            }
            _ =>  {
                panic!("Unexpected input for definitions: {:?}", p);
            }
        }
    }

    let defs = AstNode::Definitions(def_nodes);

    return defs;
}