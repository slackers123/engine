use std::fs;

use cmder::Program;

use toml::Value;

extern crate tar_script;
use tar_script::bcvm::run_file_checked;

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