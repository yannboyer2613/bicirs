use std::io::{self, Read, Write};
use std::fs::File;
use crate::lexer::Lexer;

// Defining the IR Operations Kinds.
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum IROpKind {
    IncPt, // INCREMENT THE POINTER BY 1
    DecPt, // DECREMENT THE POINTER BY 1
    IncBy, // INCREMENT BYTE BY 1
    DecBy, // DECREMENT BYTE BY 1
    OutBy, // Print THE BYTE
    ReadBy, // READ THE BYTE FROM STDIN
    JmpZe, // JUMP IF ZERO
    JmpNze, // JUMP IF NOT ZERO
}

#[derive(Clone, Copy)]
pub struct IROp {
    kind: IROpKind,
    operand: Option<u8>,
}

// Not ideal...
const TOTAL_MEMORY_CELLS: usize = 100000;

pub struct Interpreter {
    memory: [u8; TOTAL_MEMORY_CELLS],
    inst_ptr: usize,
    mem_ptr: usize,
    program: Vec<IROp>,
    lexer: Lexer,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            memory: [0; TOTAL_MEMORY_CELLS],
            inst_ptr: 0,
            mem_ptr: 0,
            program: Vec::new(),
            lexer: Lexer::new(),
        }
    }

    //fn generate_

    pub fn convert_program_to_ir_ops(&mut self, program_path: &str) {
        let mut program_file = File::open(program_path).expect("[ERROR] Unable to open the file !");
        let mut program_buffer = String::new();
        program_file.read_to_string(&mut program_buffer).expect("[ERROR] Unable to read the program !");

        self.lexer.fill(program_buffer.as_str());

        // Convert the individual instructions into IR Ops.
        /* c stands for potential operation, because we don't know if the character
         * is a valid instruction or a comment or anything else. */
        let mut c = self.lexer.next();
        while c != '@' {
            let op_kind: IROpKind;
            let op: IROp;
            match c {
                '>' | '<' | '+' | '-' | '.' => {
                    if c == '>' { op_kind = IROpKind::IncPt; }
                    else if c == '<' { op_kind = IROpKind::DecPt; }
                    else if c == '+' { op_kind = IROpKind::IncBy; }
                    else if c == '-' { op_kind = IROpKind::DecBy; }
                    else { op_kind = IROpKind::OutBy; }
                    let mut i_combo: u8 = 1; // We already have one occurence of the instruction.

                    let mut s = self.lexer.next();

                    while s == c {
                        i_combo += 1;
                        s = self.lexer.next();
                    }

                    op = IROp { kind: op_kind, operand: Some(i_combo) };
                    c = s;
                },
                ',' | '[' | ']' => {
                    if c == ',' { op_kind = IROpKind::ReadBy; }
                    else if c == '[' { op_kind = IROpKind::JmpZe; }
                    else { op_kind = IROpKind::JmpNze; }

                    op = IROp { kind: op_kind, operand: None };

                    c = self.lexer.next();
                },
                _ => continue, // We encountered invalid instruction or it's just a comment, ignore it.
            }

            self.program.push(op);
        }
    }

    pub fn interpret(&mut self) {
        let program_len = self.program.len();
        let mut stack = Vec::<usize>::new();
        let mut targets = Vec::<usize>::new();

        // I'm really dumb but hey it works...
        for _ in 0..program_len {
            targets.push(0);
        }

        // Precomputing the jumps, not optimal but it works well enough for my needs...
        let mut j: usize;
        for i in 0..program_len {
            if self.program[i].kind == IROpKind::JmpZe {
                stack.push(i);
            } else if self.program[i].kind == IROpKind::JmpNze {
                if stack.is_empty() {
                    panic!("[FATAL ERROR] Unmatched ']' at byte {}", i + 1);
                } else {
                    j = stack.pop().unwrap();
                    targets[i] = j;
                    targets[j] = i;
                }
            }
        }

        // Now it's time to execute the instructions...
        while self.inst_ptr < program_len {
            let op_to_execute = self.program[self.inst_ptr];

            match op_to_execute.kind {
                IROpKind::IncPt => self.mem_ptr += op_to_execute.operand.unwrap() as usize,
                IROpKind::DecPt => self.mem_ptr -= op_to_execute.operand.unwrap() as usize,
                IROpKind::IncBy => self.memory[self.mem_ptr] += op_to_execute.operand.unwrap(),
                IROpKind::DecBy => self.memory[self.mem_ptr] -= op_to_execute.operand.unwrap(),
                IROpKind::OutBy => {
                    let repeat_count: u8 = op_to_execute.operand.unwrap();
                    let byte_to_print_as_char = self.memory[self.mem_ptr] as char;

                    for _ in 0..repeat_count {
                        print!("{byte_to_print_as_char}");
                        io::stdout().flush().unwrap(); // Yes... flushing for every character...
                    }
                },
                IROpKind::ReadBy => {
                    let mut input: [u8; 1] = [0; 1];
                    io::stdin().read_exact(&mut input).expect("[FATAL ERROR] Unable to read stdin !");
                    self.memory[self.mem_ptr] = input[0];
                },
                IROpKind::JmpZe => {
                    if self.memory[self.mem_ptr] == 0 {
                        self.inst_ptr = targets[self.inst_ptr];
                    }
                },
                IROpKind::JmpNze => {
                    if self.memory[self.mem_ptr] != 0 {
                        self.inst_ptr = targets[self.inst_ptr];
                    }
                }
            }

            self.inst_ptr += 1;
        }
    }
}
