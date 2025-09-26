pub struct Lexer {
    code_pos: usize,
    code_len: usize,
    code: Vec<char>,
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            code_pos: 0,
            code_len: 0,
            code: Vec::new(),
        }
    }

    pub fn fill(&mut self, code: &str) {
        for c in code.chars() {
            self.code.push(c);
        }
        self.code_len = self.code.len();
    }

    fn is_this_a_valid_instruction(&self, instruction: char) -> bool {
        const VALID_INSTS: &str = "><+-.,[]";
        if VALID_INSTS.contains(instruction) { return true; }
        else { return false; }
    }

    pub fn next(&mut self) -> char {
        while self.code_pos < self.code_len && !self.is_this_a_valid_instruction(self.code[self.code_pos]) {
            self.code_pos += 1;
        }

        if self.code_pos >= self.code_len { return '@'; } // TODO: Better EOF handling...
        let char_to_return = self.code[self.code_pos];
        self.code_pos += 1;
        return char_to_return;
    }
}
