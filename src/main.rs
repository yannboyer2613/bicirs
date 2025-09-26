use bicirs::interpreter::Interpreter;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() <= 1 {
        eprintln!("[FATAL ERROR] No program provided !");
        eprintln!("[INFO] Usage : ./bicirs my_program.bf");
        std::process::exit(1);
    }

    let mut my_interpreter = Interpreter::new();

    let program_path = &args[1];

    my_interpreter.convert_program_to_ir_ops(program_path);
    my_interpreter.interpret();
}
