use std::env;
use synacor_challenge::vm::{Operation, Param, Word, VM};

fn p(param: Param) -> String {
    match param {
        Param::Literal(value) => format!("{}", value),
        Param::Register(index) => format!("#{}", index),
    }
}

fn main() -> std::io::Result<()> {
    let bin_path: String = env::args().nth(1).unwrap();
    let bin = std::fs::read(bin_path)?;
    let mut bytes = bin.iter();
    let mut memory: Vec<Word> = Vec::new();

    while let Some(&low_byte) = bytes.next() {
        if let Some(&high_byte) = bytes.next() {
            memory.push(((high_byte as Word) << 8) + (low_byte as Word));
        }
    }

    let mut vm = VM::new(memory);

    loop {
        match vm.get_next_operation() {
            Ok(operation) => {
                print!("{}: ", vm.get_ip());
                match operation {
                    Operation::Halt => println!("halt"),
                    Operation::SetRegister(register, value) => {
                        println!("setr({}, {})", p(register), p(value))
                    }
                    Operation::Push(value) => println!("push({})", p(value)),
                    Operation::Pop(output) => println!("pop({})", p(output)),
                    Operation::Equal(output, a, b) => {
                        println!("eq({}, {}, {})", p(output), p(a), p(b))
                    }
                    Operation::GreaterThan(output, a, b) => {
                        println!("gt({}, {}, {})", p(output), p(a), p(b))
                    }
                    Operation::Jump(to) => println!("jmp({})", p(to)),
                    Operation::JumpIfTrue(condition, to) => {
                        println!("jit({}, {})", p(condition), p(to))
                    }
                    Operation::JumpIfFalse(condition, to) => {
                        println!("jif({}, {})", p(condition), p(to))
                    }
                    Operation::Add(output, a, b) => {
                        println!("add({}, {}, {})", p(output), p(a), p(b))
                    }
                    Operation::Mult(output, a, b) => {
                        println!("mul({}, {}, {})", p(output), p(a), p(b))
                    }
                    Operation::Mod(output, a, b) => {
                        println!("mod({}, {}, {})", p(output), p(a), p(b))
                    }
                    Operation::And(output, a, b) => {
                        println!("and({}, {}, {})", p(output), p(a), p(b))
                    }
                    Operation::Or(output, a, b) => {
                        println!("or({}, {}, {})", p(output), p(a), p(b))
                    }
                    Operation::Not(output, a) => println!("not({}, {})", p(output), p(a)),
                    Operation::ReadMemory(output, location) => {
                        println!("rmem({}, {})", p(output), p(location))
                    }
                    Operation::WriteMemory(output, value) => {
                        println!("wmem({}, {})", p(output), p(value))
                    }
                    Operation::Call(to) => println!("call({})", p(to)),
                    Operation::Return => println!("ret"),
                    Operation::Out(Param::Literal(value)) => {
                        println!("out({})", (value as u8 as char))
                    }
                    Operation::Out(_) => println!("out(ERROR)"),
                    Operation::In(output) => println!("in({})", p(output)),
                    Operation::NoOp => println!("noop"),
                };
            }
            Err(message) => println!("ERROR: {}", message),
        }
    }

    Ok(())
}
