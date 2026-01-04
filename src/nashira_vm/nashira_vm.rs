use super::value::VALUE;
use super::instruction::TOKEN;

struct NashiraVM {
    stack: Vec<VALUE>,
    instruction_pointer: usize,
}

impl NashiraVM {
    pub fn execute(&mut self, instructions: Vec<TOKEN>) {
        loop {
            let token = instructions[self.instruction_pointer];
            self.instruction_pointer += 1;

            match token {
                TOKEN::ADD => {
                    let b = self.stack.pop();
                    let a = self.stack.pop();

                    //self.stack.push(VALUE::FLOAT + b);
                },
                TOKEN::SUB => {},
                TOKEN::MUL => {},
                TOKEN::DIV => {},
            }
        }
    }
}