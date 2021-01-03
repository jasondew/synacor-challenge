use std::collections::VecDeque;

const MOD: u16 = 32_768;
const MAX_CYCLES: u32 = 5_000_000;
pub type Word = u16;

#[derive(Debug)]
pub struct VM {
    state: State,
    cycles: u32,
    registers: Vec<Word>,
    stack: Vec<Word>,
    memory: Vec<Word>,
    ip: usize,
    input: VecDeque<Word>,
    output: Vec<char>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum State {
    Initialized,
    Running,
    WaitingForInput,
    Errored(String),
    Halted,
}

#[derive(Debug)]
pub enum Operation {
    Halt,
    SetRegister(Param, Param),
    Push(Param),
    Pop(Param),
    Equal(Param, Param, Param),
    GreaterThan(Param, Param, Param),
    Jump(Param),
    JumpIfTrue(Param, Param),
    JumpIfFalse(Param, Param),
    Add(Param, Param, Param),
    Mult(Param, Param, Param),
    Mod(Param, Param, Param),
    And(Param, Param, Param),
    Or(Param, Param, Param),
    Not(Param, Param),
    ReadMemory(Param, Param),
    WriteMemory(Param, Param),
    Call(Param),
    Return,
    Out(Param),
    In(Param),
    NoOp,
}

#[derive(Debug)]
pub enum Param {
    Literal(Word),
    Register(usize),
}

impl VM {
    pub fn new(memory: Vec<Word>) -> Self {
        Self {
            state: State::Initialized,
            cycles: 0,
            registers: vec![0; 8],
            stack: Vec::new(),
            memory: memory,
            ip: 0,
            input: VecDeque::new(),
            output: vec![],
        }
    }

    pub fn add_input(&mut self, value: Word) {
        self.input.push_back(value);
    }

    pub fn get_state(&self) -> State {
        self.state.clone()
    }

    pub fn get_cycles(&self) -> u32 {
        self.cycles
    }

    pub fn get_ip(&self) -> usize {
        self.ip
    }

    pub fn get_output(&mut self) -> String {
        let output_string: String = self.output.iter().collect();
        self.output = Vec::new();
        output_string
    }

    pub fn run(&mut self) {
        self.state = State::Running;

        loop {
            if self.state != State::Running {
                break;
            }

            if self.cycles >= MAX_CYCLES {
                self.error("reached max cycles".to_owned());
                break;
            }

            self.cycles += 1;
            match self.get_next_operation() {
                Ok(operation) => match operation {
                    Operation::Halt => self.halt(),
                    Operation::SetRegister(register, value) => self.set(register, self.get(value)),
                    Operation::Push(value) => self.push(self.get(value)),
                    Operation::Pop(output) => {
                        if let Some(value) = self.pop() {
                            self.set(output, value)
                        } else {
                            self.error("attempted to pop an empty stack!".to_owned())
                        }
                    }
                    Operation::Equal(output, a, b) => {
                        if self.get(a) == self.get(b) {
                            self.set(output, 1)
                        } else {
                            self.set(output, 0)
                        }
                    }
                    Operation::GreaterThan(output, a, b) => {
                        if self.get(a) > self.get(b) {
                            self.set(output, 1)
                        } else {
                            self.set(output, 0)
                        }
                    }
                    Operation::Jump(to) => self.jump(to),
                    Operation::JumpIfTrue(condition, to) => {
                        if self.get(condition) > 0 {
                            self.jump(to)
                        }
                    }
                    Operation::JumpIfFalse(condition, to) => {
                        if self.get(condition) == 0 {
                            self.jump(to)
                        }
                    }
                    Operation::Add(output, a, b) => {
                        self.set(output, (self.get(a).wrapping_add(self.get(b))) % MOD)
                    }
                    Operation::Mult(output, a, b) => {
                        self.set(output, (self.get(a).wrapping_mul(self.get(b))) % MOD)
                    }
                    Operation::Mod(output, a, b) => {
                        self.set(output, (self.get(a) % self.get(b)) % MOD)
                    }
                    Operation::And(output, a, b) => self.set(output, self.get(a) & self.get(b)),
                    Operation::Or(output, a, b) => self.set(output, self.get(a) | self.get(b)),
                    Operation::Not(output, a) => self.set(output, self.get(a) ^ 0b111111111111111),
                    Operation::ReadMemory(output, location) => {
                        self.set(output, self.get_memory(location))
                    }
                    Operation::WriteMemory(output, value) => {
                        self.set_memory(output, self.get(value))
                    }
                    Operation::Call(to) => {
                        self.push(self.ip as u16);
                        self.jump(to);
                    }
                    Operation::Return => {
                        if let Some(value) = self.pop() {
                            self.jump(Param::Literal(value));
                        } else {
                            self.halt();
                        }
                    }
                    Operation::Out(value) => self.output.push(self.get(value) as u8 as char),
                    Operation::In(output) => {
                        if let Some(value) = self.input.pop_front() {
                            self.set(output, value);
                        } else {
                            self.ip -= 2;
                            self.state = State::WaitingForInput;
                        }
                    }
                    Operation::NoOp => {}
                },
                Err(message) => self.error(message),
            }
        }
    }

    pub fn get_next_operation(&mut self) -> Result<Operation, String> {
        let opcode = self.memory[self.ip];
        self.ip += 1;

        match opcode {
            0 => Ok(Operation::Halt),
            1 => Ok(Operation::SetRegister(self.read_param(), self.read_param())),
            2 => Ok(Operation::Push(self.read_param())),
            3 => Ok(Operation::Pop(self.read_param())),
            4 => Ok(Operation::Equal(
                self.read_param(),
                self.read_param(),
                self.read_param(),
            )),
            5 => Ok(Operation::GreaterThan(
                self.read_param(),
                self.read_param(),
                self.read_param(),
            )),
            6 => Ok(Operation::Jump(self.read_param())),
            7 => Ok(Operation::JumpIfTrue(self.read_param(), self.read_param())),
            8 => Ok(Operation::JumpIfFalse(self.read_param(), self.read_param())),
            9 => Ok(Operation::Add(
                self.read_param(),
                self.read_param(),
                self.read_param(),
            )),
            10 => Ok(Operation::Mult(
                self.read_param(),
                self.read_param(),
                self.read_param(),
            )),
            11 => Ok(Operation::Mod(
                self.read_param(),
                self.read_param(),
                self.read_param(),
            )),
            12 => Ok(Operation::And(
                self.read_param(),
                self.read_param(),
                self.read_param(),
            )),
            13 => Ok(Operation::Or(
                self.read_param(),
                self.read_param(),
                self.read_param(),
            )),
            14 => Ok(Operation::Not(self.read_param(), self.read_param())),
            15 => Ok(Operation::ReadMemory(self.read_param(), self.read_param())),
            16 => Ok(Operation::WriteMemory(self.read_param(), self.read_param())),
            17 => Ok(Operation::Call(self.read_param())),
            18 => Ok(Operation::Return),
            19 => Ok(Operation::Out(self.read_param())),
            20 => Ok(Operation::In(self.read_param())),
            21 => Ok(Operation::NoOp),
            opcode => Err(format!("unknown opcode: {}", opcode)),
        }
    }

    fn read_param(&mut self) -> Param {
        let word = self.memory[self.ip];
        self.ip += 1;

        if word < MOD {
            Param::Literal(word)
        } else {
            // TODO: ensure that we aren't trying to target an invalid register
            let register = word - MOD;
            Param::Register(register as usize)
        }
    }

    fn get(&self, param: Param) -> Word {
        match param {
            Param::Literal(value) => value,
            Param::Register(index) => self.registers[index],
        }
    }

    fn get_memory(&self, location: Param) -> Word {
        self.memory[self.get(location) as usize]
    }

    fn set(&mut self, param: Param, value: Word) {
        match param {
            Param::Register(index) => self.registers[index] = value,
            Param::Literal(_) => self.error("attempted to write to a literal".to_owned()),
        }
    }

    fn set_memory(&mut self, location: Param, value: Word) {
        let index = self.get(location) as usize;

        self.memory[index] = value
    }

    fn push(&mut self, value: Word) {
        self.stack.push(value)
    }

    fn pop(&mut self) -> Option<Word> {
        self.stack.pop()
    }

    fn jump(&mut self, to: Param) {
        self.ip = self.get(to) as usize
    }

    fn error(&mut self, error: String) {
        self.state = State::Errored(error);
    }

    fn halt(&mut self) {
        self.state = State::Halted;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_test() {
        let mut vm = VM::new(vec![0]);
        assert_eq!(vm.state, State::Initialized);

        vm.run();
        assert_eq!(vm.state, State::Halted);
    }

    #[test]
    fn jump_test() {
        let mut vm = VM::new(vec![6, 3, 42, 7, 2, 6, 8, 2, 10, 0]);
        vm.run();
        assert_eq!(vm.state, State::Halted);
        assert_eq!(vm.cycles, 4);
        assert_eq!(vm.ip, 10);
    }

    #[test]
    fn not_test() {
        let mut vm = VM::new(vec![14, 32768, 32767, 16, 6, 32768, 42]);
        vm.run();
        assert_eq!(vm.state, State::Halted);
        assert_eq!(vm.cycles, 3);
        dbg!(&vm.memory);
        assert_eq!(vm.memory[6], 0);
    }

    #[test]
    fn add_and_output_test() {
        let mut vm = VM::new(vec![9, 32768, 32769, 88, 19, 32768, 0]);
        vm.run();

        assert_eq!(vm.state, State::Halted);
        assert_eq!(vm.get_output(), "X");
    }
}
