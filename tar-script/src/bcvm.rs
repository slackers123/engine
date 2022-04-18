use std::fmt::Debug;
use std::collections::HashMap;
use std::fs;

use serde::{Serialize, Deserialize};
use serde_json::Result;

const MAX_RECURSION_DEPTH: u8 = 200;
#[non_exhaustive]
struct BCInst;

impl BCInst {
    pub const PRINT: u8             = 0x00; // 0    // prints the top val
    pub const LOAD_CONST: u8        = 0x01; // 1    // loads a local const
    pub const ADD: u8               = 0x02; // 0    // adds the top to numbers and pushes the result
    pub const SUB: u8               = 0x03; // 0    // subtracts the top to numbers and pushes the result
    pub const EQUAL: u8             = 0x07; // 0    // checks if the last two values are qual
    pub const GREATER_THAN: u8      = 0x08; // 0    // checks if the second to top value is greater than the top value and pushes the result
    pub const LESS_THAN: u8         = 0x09; // 0    // checks if the second to top value is less than the top value and pushes the result
    pub const OR: u8                = 0x0a; // 0    // the last two values have to be bools; ors the last two vals and pushes the result
    pub const AND: u8               = 0x0b; // 0    // the last two values have to be bools; ands the last two vals and pushes the result
    pub const NOT: u8               = 0x0c; // 0    // the last val has to be a bool; inverts the last val on the stack
    pub const STORE_LOCAL_VAL: u8   = 0x04; // 1    // stores the top val in the local env
    pub const LOAD_LOCAL_VAL: u8    = 0x05; // 1    // pushes the specified local env val
    pub const CALL_FUNC: u8         = 0x06; // 0    // has to be called after the function args have been pushed onto the stack; calls function named ontop of the stack
    pub const JUMP: u8              = 0x10; // 2-?  // jumps to the value specified; all jumps: 1st word is 0 -> jump forward; first word is 1 -> jump backward; second word: how many bytes follow (these encode the actual distance can be 1 2 4 or 8)
    pub const JUMP_IF_TRUE: u8      = 0x0d; // 2-?  // jumps by the amount specified if the top value on the stack is true
    pub const JUMP_IF_FALSE: u8     = 0x0e; // 2-?  // jumps by the amount specified if the top value on the stack is false
    pub const NOP: u8               = 0x0f; // 0    // empty operation
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Val {
    Int(i32),
    Long(i64),
    Float(f64),
    String(String),
    Bool(bool),
}

fn print_val(val: Val) {
    match val {
        Val::Int(v) => {
            println!("{}", v);
        }
        Val::Long(v) => {
            println!("{}", v);
        }
        Val::Float(v) => {
            println!("{}", v);
        }
        Val::String(v) => {
            println!("{}", v);
        }
        Val::Bool(v) => {
            println!("{}", v);
        }
    }
}

#[allow(unused)]
pub fn store_bc(path: String, funcs: HashMap<String, (Vec<u8>, Vec<Val>)>, stack: Vec<Val>, entry: String) {
    let j = serde_json::to_string(&(funcs.clone(), stack.clone(), "main".to_owned())).unwrap();

    fs::write(path, j).unwrap();
}

pub fn run_from_string(string: String) -> Result<()> {
    let (funcs, mut stack, entry): (HashMap<String, (Vec<u8>, Vec<Val>)>, Vec<Val>, String) = serde_json::from_str(string.as_str())?;

    run_func(entry, &funcs, &mut stack, 0);

    return Ok(());
}

pub fn run_func(name: String, funcs: &HashMap<String, (Vec<u8>, Vec<Val>)>, stack: &mut Vec<Val>, d: u8) {
    let d = d+1;
    if d > MAX_RECURSION_DEPTH {
        panic!("stack overflow error")
    }

    let (func, consts) = funcs.get(&name).expect(format!("non existent function: {}", name).as_str());
    
    let mut env: Vec<Val>   = vec![];

    let mut i: usize = 0;
    while i < func.len() {
        match func[i] {
            BCInst::PRINT => {
                print_val(stack.pop().expect("missing val on stack for print operation"))
            }
            BCInst::LOAD_CONST => {
                stack.push(consts[func[i+1] as usize].clone());
                i+=1;
            }
            BCInst::ADD => {
                let val1 = stack.pop().expect("not enough vals on stack for add op.");
                let val2 = stack.pop().expect("not enough vals on stack for add op.");

                match val1 {
                    Val::Int(v) => {
                        if let Val::Int(v1) = val2 {
                            stack.push(Val::Int(v.checked_add(v1).expect("addition with integer overflow")));
                        }
                        else {
                            panic!("can only add the same types");
                        }
                    }

                    Val::Long(v) => {
                        if let Val::Long(v1) = val2 {
                            stack.push(Val::Long(v.checked_add(v1).expect("addition with integer overflow")));
                        }
                        else {
                            panic!("can only add the same types");
                        }
                    }

                    Val::Float(v) => {
                        if let Val::Float(v1) = val2 {
                            stack.push(Val::Float(v + v1));
                        }
                        else {
                            panic!("can only add the same types");
                        }
                    }

                    Val::String(v) => {
                        if let Val::String(v1) = val2 {
                            stack.push(Val::String(v + v1.as_str()));
                        }
                        else {
                            panic!("can only add the same types");
                        }
                    }

                    Val::Bool(_) => {
                        panic!("cannot add bools");
                    }
                }
            }
            BCInst::SUB => {
                let val1 = stack.pop().expect("not enough vals on stack for subtraction op.");
                let val2 = stack.pop().expect("not enough vals on stack for subtraction op.");

                match val1 {
                    Val::Int(v) => {
                        if let Val::Int(v1) = val2 {
                            stack.push(Val::Int(v.checked_sub(v1).expect("subtraction with overflow")));
                        }
                        else {
                            panic!("can only add the same types");
                        }
                    }

                    Val::Long(v) => {
                        if let Val::Long(v1) = val2 {
                            stack.push(Val::Long(v.checked_sub(v1).expect("subtraction with overflow")));
                        }
                        else {
                            panic!("can only add the same types");
                        }
                    }

                    Val::Float(v) => {
                        if let Val::Float(v1) = val2 {
                            stack.push(Val::Float(v - v1));
                        }
                        else {
                            panic!("can only add the same types");
                        }
                    }

                    Val::String(_) => {
                        panic!("cannot perform subtraction on strings");
                    }

                    Val::Bool(_) => {
                        panic!("cannot subtract bools");
                    }
                }
            }

            BCInst::STORE_LOCAL_VAL => {
                let val = stack.pop().expect("not enough vals");
                let idx = func[i+1] as usize;
                if idx < env.len() {
                    env[idx] = val;
                }
                else if idx == env.len() {
                    env.push(val);
                }
                else {
                    panic!("random access to env not allowed");
                }
                i+=1;
            }

            BCInst::LOAD_LOCAL_VAL => {
                let val = env[func[i+1] as usize].clone();
                stack.push(val);
                i+=1;
                let temp = stack.pop().unwrap();
                stack.push(temp);
            }

            BCInst::CALL_FUNC => {
                if let Val::String(v) = stack.pop().expect("not enough vals") {
                    run_func(v, funcs, stack, d);
                }
                else {
                    panic!("the top stack val has to be a string for CALL_FUNC");
                }
            }

            BCInst::EQUAL => {
                let val2 = stack.pop().expect("not enough vals on stack for equal op.");
                let val1 = stack.pop().expect("not enough vals on stack for equal op.");

                match val1 {
                    Val::Int(v) => {
                        if let Val::Int(v1) = val2 {
                            stack.push(Val::Bool(v == v1));
                        }
                        else {
                            panic!("can only add the same types");
                        }
                    }

                    Val::Long(v) => {
                        if let Val::Long(v1) = val2 {
                            stack.push(Val::Bool(v == v1));
                        }
                        else {
                            panic!("can only add the same types");
                        }
                    }

                    Val::Float(v) => {
                        if let Val::Float(v1) = val2 {
                            stack.push(Val::Bool(v == v1));
                        }
                        else {
                            panic!("can only add the same types");
                        }
                    }

                    Val::String(v) => {
                        if let Val::String(v1) = val2 {
                            stack.push(Val::Bool(v == v1));
                        }
                        else {
                            panic!("can only add the same types");
                        }
                    }

                    Val::Bool(v) => {
                        if let Val::Bool(v1) = val2 {
                            stack.push(Val::Bool(v == v1));
                        }
                        else {
                            panic!("can only add the same types");
                        }
                    }
                }
            }

            BCInst::GREATER_THAN => {
                let val2 = stack.pop().expect("not enough vals on stack for greater than op.");
                let val1 = stack.pop().expect("not enough vals on stack for greater than op.");

                match val1 {
                    Val::Int(v) => {
                        if let Val::Int(v1) = val2 {
                            stack.push(Val::Bool(v > v1));
                        }
                        else {
                            panic!("can only add the same types");
                        }
                    }

                    Val::Long(v) => {
                        if let Val::Long(v1) = val2 {
                            stack.push(Val::Bool(v > v1));
                        }
                        else {
                            panic!("can only add the same types");
                        }
                    }

                    Val::Float(v) => {
                        if let Val::Float(v1) = val2 {
                            stack.push(Val::Bool(v > v1));
                        }
                        else {
                            panic!("can only add the same types");
                        }
                    }

                    Val::String(_) => {
                        panic!("cannot compare strings");
                    }

                    Val::Bool(_) => {
                        panic!("cannot compare bools");
                    }
                }
            }

            BCInst::LESS_THAN => {
                let val2 = stack.pop().expect("not enough vals on stack for less than op.");
                let val1 = stack.pop().expect("not enough vals on stack for less than op.");

                match val1 {
                    Val::Int(v) => {
                        if let Val::Int(v1) = val2 {
                            stack.push(Val::Bool(v < v1));
                        }
                        else {
                            panic!("can only add the same types");
                        }
                    }

                    Val::Long(v) => {
                        if let Val::Long(v1) = val2 {
                            stack.push(Val::Bool(v < v1));
                        }
                        else {
                            panic!("can only add the same types");
                        }
                    }

                    Val::Float(v) => {
                        if let Val::Float(v1) = val2 {
                            stack.push(Val::Bool(v < v1));
                        }
                        else {
                            panic!("can only add the same types");
                        }
                    }

                    Val::String(_) => {
                        panic!("cannot compare strings");
                    }

                    Val::Bool(_) => {
                        panic!("cannot compare bools");
                    }
                }
            }

            BCInst::OR => {
                if let Val::Bool(v) = stack.pop().expect("stack too short") {
                    if let Val::Bool(v1) = stack.pop().expect("stack too short") {
                        stack.push(Val::Bool(v || v1));
                    }
                }
                else {
                    panic!("top val has to be bool!");
                }
            }

            BCInst::AND => {
                if let Val::Bool(v) = stack.pop().expect("stack too short") {
                    if let Val::Bool(v1) = stack.pop().expect("stack too short") {
                        stack.push(Val::Bool(v && v1));
                    }
                }
                else {
                    panic!("top val has to be bool!");
                }
            }

            BCInst::NOT => {
                if let Val::Bool(v) = stack.pop().expect("stack too short") {
                    stack.push(Val::Bool(!v));
                }
                else {
                    panic!("top val has to be bool!");
                }
            }

            BCInst::JUMP_IF_TRUE => {
                if let Val::Bool(v) = stack.pop().expect("stack too short") {
                    let (dist, len, go_fwd) = get_jump_dist(&func, i);
                    if v {
                        if go_fwd {
                            i += dist;
                        }
                        else {
                            i-= dist;
                        }
                    }
                    else {
                        i+= len+1;
                    }
                }
                else {
                    panic!("top val has to be bool!");
                } 
            }

            BCInst::JUMP_IF_FALSE => {
                if let Val::Bool(v) = stack.pop().expect("stack too short") {
                    let (dist, len, go_fwd) = get_jump_dist(&func, i);
                    if !v {
                        if go_fwd {
                            i += dist;
                        }
                        else {
                            i-= dist;
                        }
                    }
                    else {
                        i+= len+1;
                    }
                }
                else {
                    panic!("top val has to be bool!");
                }
            }

            BCInst::JUMP => {
                let (dist, _, go_fwd) = get_jump_dist(&func, i);

                if go_fwd {
                    i += dist;
                }
                else {
                    i-= dist+1;
                }
            }

            BCInst::NOP => {}

            _ => panic!("unknown byte code: {}", func[i])
        }
        i+=1;
    }
}

fn get_jump_dist(func: &Vec<u8>, i: usize) -> (usize, usize, bool) {
    let a = ((func[i+1]) & (240))>>4;
    let b = (func[i+1]) & (15);
    if a == 0 {
        match b {
            1 => {return ((func[i+2] + 1) as usize, b as usize, true)}

            2 => {
                let inc = ((func[i+2] as u16)<<8) | func[i+3] as u16;
                return ((inc + 1) as usize, b as usize, true);
            }

            4 => {
                let inc = ((func[i+2] as u32)<<26) | ((func[i+3] as u32)<<16) | ((func[i+4] as u32)<<8) | (func[i+5] as u32);
                return ((inc + 1) as usize, b as usize, true);
            }

            8 => {
                let mut inc: u64 = 0;
                for j in 2..10 {
                    inc = inc | (func[j] as u64)<<(64-(8*(j-1)));
                }
                return ((inc + 1) as usize, b as usize, true);
            }

            _=>{panic!("incorrect input to jump function")}
        }
    }
    else if a == 1 {
        match b {
            1 => {return ((func[i+2] + 1) as usize, b as usize, false)}

            2 => {
                let inc = ((func[i+2] as u16)<<8) | func[i+3] as u16;
                return ((inc + 1) as usize, b as usize, false);
            }

            4 => {
                let inc = ((func[i+2] as u32)<<26) | ((func[i+3] as u32)<<16) | ((func[i+4] as u32)<<8) | (func[i+5] as u32);
                return ((inc + 1) as usize, b as usize, false);
            }

            8 => {
                let mut inc: u64 = 0;
                for j in 2..10 {
                    inc = inc | (func[j] as u64)<<(64-(8*(j-1)));
                }
                return ((inc + 1) as usize, b as usize, false);
            }

            _=>{panic!("incorrect input to jump function")}
        }
    }
    else {panic!("incorrect input to jump function")}
}