use std::fs;
use std::collections::HashMap;

use cmder::Program;

use toml::Value;

extern crate tar_script;
use tar_script::bcvm::run_file_checked;
use tar_script::bcvm::Val;
use tar_script::bcvm::BCInst;

// use tar_script::bcvm;

#[derive(Clone, Debug)]
pub enum InterpMethod {
    AstWalking,
    Bytecode,
    Jit,
    Asm,
}

#[derive(Clone, Debug)]
pub struct CompilationOptions {
    interp_method: InterpMethod,
    path: String,
}

impl CompilationOptions {
    fn path(&mut self, p: String) -> CompilationOptions {
        self.path = p;
        return self.clone();
    }
}

impl Default for CompilationOptions {
    fn default() -> Self {
        CompilationOptions {
            interp_method: InterpMethod::Bytecode,
            path: "src/main.tar".to_owned(),
        }
    }
}

fn start_project_compilation(options: CompilationOptions) {
    match options.interp_method {
        InterpMethod::Bytecode => {
            run_file_checked(options.path.as_str()).unwrap();
        }
        InterpMethod::AstWalking => {

        }

        _ => {
            panic!("the requested execution method is not available yet.");
        }
    }
}


fn main() {
    let mut program = Program::new();

    program
        .version("0.1.0")
        .description("tar-script manager");

    program
        .command("run")
        .alias("r")
        .description("the run command")
        .option("-l --location", "specify a location other than the src directory for the main file")
        .action(|_vals, _opts| {
            let config_f = fs::read_to_string("tar.toml").expect("there is no 'tar.toml' file in this directory (you can use the -l flag to specify a different location;");

            let config = config_f.parse::<Value>().unwrap();
            if let Value::Table(t) = config {
                let _deps = t.get("dependencies").unwrap();
                let info = t.get("project").unwrap();
                if let Value::Table(t) = info {
                    let path = t.get("entry").unwrap();
                    println!("executing program in: {}", path);
                    if let Value::String(s) = path {
                        start_project_compilation(CompilationOptions::default().path(s.clone()));
                    }
                }
            }
        })
        .build(&mut program);

    program
    .command("bin")
    .alias("b")
    .description("the rrun binary command")
    .option("-l --location", "specify a location other than the current directory for the project")
    .action(|vals, opts| {
        let mut fns: HashMap<String, (Vec<u8>, Vec<Val>)> = HashMap::new();

        // conv_base(nr1: int[], b1: int, b2: int) -> int[] {
        //      int[[]] r = devide_base(nr1, b1, b2);
        //      int[] res = [];
        //      if get_int_val(r[0][0], b1) != 0 {
        //          res = conv_base(r[0], b1, b2);
        //          push:>(res, r[1]);
        //          return res;
        //      }
        //      else {
        //          return [get_int_val([r[1]], b2)];
        //      }
        // }
        let conv_base = vec![
            BCInst::STORE_LOCAL_VAL, // nr1: 0
            BCInst::STORE_LOCAL_VAL, // b1: 1
            BCInst::STORE_LOCAL_VAL, // b2: 2

            BCInst::LOAD_LOCAL_VAL, 0,
            BCInst::LOAD_LOCAL_VAL, 1,
            BCInst::LOAD_LOCAL_VAL, 2,
            BCInst::LOAD_CONST, 0,
            BCInst::CALL_FUNC,
            BCInst::STORE_LOCAL_VAL, // r: 3

            BCInst::LOAD_CONST, 1,
            BCInst::STORE_LOCAL_VAL, // res: 4

            BCInst::LOAD_LOCAL_VAL, 1,
            BCInst::LOAD_LOCAL_VAL, 3,
            BCInst::LOAD_CONST, 2, 
            BCInst::GET_ARR_AT,
            BCInst::LOAD_CONST, 2, 
            BCInst::GET_ARR_AT, 
            BCInst::LOAD_CONST, 3,
            BCInst::CALL_FUNC,
            BCInst::LOAD_CONST, 2,
            BCInst::EQUAL,
            BCInst::JUMP_IF_FALSE, 0x01, 22, // recalculate

            BCInst::LOAD_LOCAL_VAL, 2,
            BCInst::LOAD_LOCAL_VAL, 1,
            BCInst::LOAD_LOCAL_VAL, 3,
            BCInst::LOAD_CONST, 2, 
            BCInst::GET_ARR_AT,
            BCInst::LOAD_CONST, 4,
            BCInst::CALL_FUNC,

            BCInst::LOAD_LOCAL_VAL, 3,
            BCInst::LOAD_CONST, 5, 
            BCInst::GET_ARR_AT,
            BCInst::PUSH_TO_ARR,

            BCInst::JUMP, 0x01, 16, // recalculate

            BCInst::LOAD_CONST, 1,
            BCInst::LOAD_LOCAL_VAL, 2,
            BCInst::LOAD_CONST, 1,
            BCInst::LOAD_LOCAL_VAL, 3,
            BCInst::LOAD_CONST, 5, 
            BCInst::GET_ARR_AT,
            BCInst::PUSH_TO_ARR,
            BCInst::LOAD_CONST, 3,
            BCInst::CALL_FUNC,
            BCInst::PUSH_TO_ARR,


        ];
        let conv_base_consts = vec![Val::String("devide_base".to_owned()), 
                                    Val::Array{ty: tar_script::bcvm::Type::Array, arr: vec![] },
                                    Val::Int(0),
                                    Val::String("get_int_val".to_owned()),
                                    Val::String("conv_base".to_owned()),
                                    Val::Int(1),
        ];
    
        // get_int_val(nr: int[], b: int) -> int {
        //      int sum = 0;
        //      int i = get_len(nr)-1;
        //      while i > 0 {
        //          sum = sum + get_at(nr, i) * pow(b, get_len(nr) - i);
        //          i = i-1;
        //      }
        //      return sum;
        // }
        let get_int_val = vec![
            BCInst::STORE_LOCAL_VAL, 0,
            BCInst::STORE_LOCAL_VAL, 1,

            BCInst::LOAD_CONST, 0,
            BCInst::STORE_LOCAL_VAL, 2, // sum
            
            BCInst::LOAD_LOCAL_VAL, 0,
            BCInst::LOAD_ARR_LEN,
            BCInst::LOAD_CONST, 2,
            BCInst::SUB,
            BCInst::STORE_LOCAL_VAL, 3, // i,
            
            BCInst::LOAD_LOCAL_VAL, 1,
            BCInst::LOAD_LOCAL_VAL, 0,
            BCInst::LOAD_ARR_LEN, 
            BCInst::LOAD_LOCAL_VAL, 3,
            BCInst::SUB,
            BCInst::LOAD_CONST, 2,
            BCInst::SUB,

            BCInst::LOAD_CONST, 1,
            BCInst::CALL_FUNC, // calls pow

            BCInst::STORE_LOCAL_VAL, 4,
            BCInst::LOAD_LOCAL_VAL, 4,
            BCInst::PRINT,
            BCInst::LOAD_LOCAL_VAL, 4,

            BCInst::LOAD_LOCAL_VAL, 0,
            BCInst::LOAD_LOCAL_VAL, 3,
            BCInst::GET_ARR_AT,
            BCInst::MUL,

            BCInst::LOAD_LOCAL_VAL, 2,
            BCInst::ADD,
            BCInst::STORE_LOCAL_VAL, 2,

            BCInst::LOAD_LOCAL_VAL, 3,
            BCInst::LOAD_CONST, 2,
            BCInst::SUB,
            BCInst::STORE_LOCAL_VAL, 3,

            BCInst::LOAD_LOCAL_VAL, 3,
            BCInst::LOAD_CONST, 0,
            BCInst::GREATER_THAN,
            BCInst::LOAD_LOCAL_VAL, 3,
            BCInst::LOAD_CONST, 0,
            BCInst::EQUAL,
            BCInst::OR,
            BCInst::JUMP_IF_TRUE, 0x11, 50, // figure out distance

            BCInst::LOAD_LOCAL_VAL, 2,

            BCInst::PRINT,
        ];

        let get_int_val_consts = vec![
            Val::Int(0),
            Val::String("pow".to_owned()),
            Val::Int(1),

        ];


        // pow(a: int, b: int) -> int {
        //      int i = 0;
        //      int tmp = a;
        //      while i < b {
        //          a = a * tmp;
        //          i = i +1;
        //      }
        //      return a;
        // }
        let pow = vec![
            BCInst::STORE_LOCAL_VAL, 0,
            BCInst::STORE_LOCAL_VAL, 1,
        
            BCInst::LOAD_CONST, 0,
            BCInst::STORE_LOCAL_VAL, 2, // i: 2

            BCInst::LOAD_LOCAL_VAL, 0,
            BCInst::STORE_LOCAL_VAL, 3, // tmp: 3

            BCInst::LOAD_LOCAL_VAL, 0,
            BCInst::LOAD_LOCAL_VAL, 3,
            BCInst::MUL,
            BCInst::STORE_LOCAL_VAL, 0,

            BCInst::LOAD_LOCAL_VAL, 2,
            BCInst::LOAD_CONST, 0,
            BCInst::ADD,
            BCInst::STORE_LOCAL_VAL, 2,

            BCInst::LOAD_LOCAL_VAL, 2,
            BCInst::LOAD_LOCAL_VAL, 1,
            BCInst::LESS_THAN,
            BCInst::JUMP_IF_TRUE, 0x11, 19,

            BCInst::LOAD_LOCAL_VAL, 0,
        ];

        let pow_consts = vec![
            Val::Int(1),
        ];
        

        fns.insert("conv_base".to_owned(), (conv_base, conv_base_consts));
        fns.insert("pow".to_owned(), (pow, pow_consts));
        fns.insert("get_int_val".to_owned(), (get_int_val, get_int_val_consts));

        tar_script::bcvm::run_func("get_int_val".to_owned(), &mut fns, &mut vec![Val::Int(2), Val::Array{ty: tar_script::bcvm::Type::Int, arr: vec![Val::Int(1), Val::Int(1)]},], 0);
        // tar_script::bcvm::run_func("pow".to_owned(), &mut fns, &mut vec![Val::Int(1), Val::Int(2)], 0);
    })
    .build(&mut program);
    

    program
        .command("new")
        .alias("n")
        .description("the new command")
        .option("-l --location", "specify a location other than the current directory for the project")
        .action(|vals, opts| {
            dbg!(vals);
            dbg!(opts);
        })
        .build(&mut program);

    program.parse();

}