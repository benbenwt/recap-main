use std::env;
use win_programs::WinProgram;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            WinProgram::list_all()
                .iter()
                .for_each(|p| println!("{}: {}", p.name(), p.exe()));
        }
        _ => {
            let program_name = &args[1];
            let args = &args[2..];
            let args_str: Vec<&str> = args.iter().map(std::string::String::as_str).collect();
            match WinProgram::new(program_name.clone()) {
                Ok(p) => {
                    p.start_with_args(&args_str).unwrap_or_else(|e| {
                        eprintln!("Failed to start program '{program_name}': {e}");
                        std::process::exit(1);
                    });
                }
                Err(e) => {
                    eprintln!("Error launching program '{program_name}': {e}");
                    std::process::exit(1);
                }
            }
        }
    }
}
