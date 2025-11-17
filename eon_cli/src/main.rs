use eon_core::FileData;
use eon_core::OsFileProvider;
use clap::Parser;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use std::process::exit;
mod host_funcs;
use host_funcs::*;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
struct Args {
    #[arg(
        help = "The main Eon file to compile and execute",
        value_name = "FILE"
    )]
    file: String,
    #[arg(
        short,
        long,
        value_name = "DIRECTORY",
        help = "Override the default module directory (~/.eon/modules)."
    )]
    modules: Option<String>,
    #[arg(
        short,
        long,
        value_name = "DIRECTORY",
        help = "Override the default shared objects directory (~/.eon/shared_objects)."
    )]
    shared_objects: Option<String>,
    #[arg(
        help = "Arguments to pass to the Eon program",
        value_name = "ARGS",
        trailing_var_arg = true
    )]
    args: Vec<String>,
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut source_files = Vec::new();
    let contents = match std::fs::read_to_string(&args.file) {
        Ok(c) => c,
        Err(err) => {
            eprintln!("Could not open file '{}': {}", args.file, err);
            exit(1);
        }
    };
    let main_file_path: PathBuf = args.file.clone().into();
    source_files.push(FileData::new(
        main_file_path.clone(),
        main_file_path.clone(),
        contents,
    ));
    source_files.push(FileData::new(
        "prelude.en".into(),
        "prelude.en".into(),
        eon_core::prelude::PRELUDE.to_string(),
    ));
    let modules_dir: PathBuf = match args.modules {
        Some(modules) => {
            let current_dir = std::env::current_dir().expect("Can't get current directory.");
            current_dir.join(modules)
        }
        None => {
            let home_dir = home::home_dir().expect("Can't get home directory.");
            home_dir.join(".eon/modules")
        }
    };
    let shared_objects_dir: PathBuf = match args.shared_objects {
        Some(shared_objects_dir) => {
            let current_dir = std::env::current_dir().expect("Can't get current directory.");
            current_dir.join(shared_objects_dir)
        }
        None => {
            let home_dir = home::home_dir().expect("Can't get home directory.");
            home_dir.join(".eon/shared_objects")
        }
    };
    let main_file_dir = if main_file_path.is_absolute() {
        main_file_path.parent().unwrap()
    } else {
        &std::env::current_dir()
            .unwrap()
            .join(main_file_path.parent().unwrap())
    };
    let file_provider = OsFileProvider::new(main_file_dir.into(), modules_dir, shared_objects_dir);
    let main_file_name = main_file_path.file_name().unwrap().to_str().unwrap();
    match eon_core::compile_bytecode(main_file_name, file_provider) {
        Ok(program) => {
            let mut vm = eon_core::vm::Vm::new(program);
            loop {
                vm.run();
                vm.gc();
                if vm.is_done() {
                    return Ok(());
                }
                if let Some(error) = vm.get_error() {
                    eprint!("{error}");
                    exit(1);
                }
                if let Some(pending_host_func) = vm.get_pending_host_func() {
                    let host_func_args: HostFunctionArgs =
                        HostFunctionArgs::from_vm(&mut vm, pending_host_func);
                    match host_func_args {
                        HostFunctionArgs::PrintString(s) => {
                            print!("{s}");
                            io::stdout().flush().unwrap();
                            HostFunctionRet::PrintString.into_vm(&mut vm);
                        }
                        HostFunctionArgs::Readline => {
                            let mut input = String::new();
                            io::stdin().read_line(&mut input).unwrap();
                            if input.ends_with('\n') {
                                input.pop();
                                if input.ends_with('\r') {
                                    input.pop();
                                }
                            }
                            HostFunctionRet::Readline(input).into_vm(&mut vm);
                        }
                    }
                }
            }
        }
        Err(err) => {
            err.emit();
            exit(1);
        }
    }
}