use std::env;
use std::io;
use std::io::prelude::*;
use synacor_challenge::vm::{State, Word, VM};

fn add_line_of_input(vm: &mut VM, line: &str) {
    for &byte in line.as_bytes() {
        vm.add_input(byte as Word)
    }
    vm.add_input(b'\n' as Word);
}

fn main() -> std::io::Result<()> {
    let bin_path: String = env::args().nth(1).unwrap();
    println!("Loading `{}`...", bin_path);

    let bin = std::fs::read(bin_path)?;
    let mut bytes = bin.iter();
    let mut memory: Vec<Word> = Vec::new();

    while let Some(&low_byte) = bytes.next() {
        if let Some(&high_byte) = bytes.next() {
            memory.push(((high_byte as Word) << 8) + (low_byte as Word));
        }
    }

    let mut vm = VM::new(memory);

    for line in vec![
        "take tablet",
        "use tablet",
        "doorway",
        "north",
        "north",
        "bridge",
        "continue",
        "down",
        "east",
        "take empty lantern",
        "west",
        "west",
        "passage",
        "ladder",
        "west",
        "south",
        "north",
        "take can",
        "use can",
        "west",
        "ladder",
        "darkness",
        "use lantern",
        "continue",
        "west",
        "west",
        "west",
        "west",
        "north",
        "take red coin",
        "look red coin",
        "north",
        "east",
        "take concave coin",
        "look concave coin",
        "west",
        "west",
        "take blue coin",
        "look blue coin",
        "east",
        "west",
        "up",
        "take shiny coin",
        "look shiny coin",
        "down",
        "east",
        "east",
        "down",
        "take corroded coin",
        "look corroded coin",
        "up",
        "west",
        "use blue coin",
        "use red coin",
        "use shiny coin",
        "use concave coin",
        "use corroded coin",
        "north",
        "take teleporter",
        "look teleporter",
        "use teleporter",
        "take business card",
        "look business card",
        "take strange book",
        "look strange book",
    ] {
        add_line_of_input(&mut vm, line);
    }

    vm.run();
    vm.fix_teleporter();

    for line in vec![
        "use teleporter",
        "north",
        "north",
        "north",
        "north",
        "north",
        "north",
        "north",
        "north",
        "north",
        "take orb",
        "look orb",
        "look",
        "north",
        "east",
        "east",
        "north",
        "west",
        "south",
        "east",
        "east",
        "west",
        "north",
        "north",
        "east",
        "vault",
        "take mirror",
        "use mirror",
    ] {
        add_line_of_input(&mut vm, line);
    }

    loop {
        vm.run();

        if vm.get_state() == State::WaitingForInput {
            println!("{}", vm.get_output());
            print!("> ");
            std::io::stdout().flush().unwrap();

            if let Some(Ok(line)) = io::stdin().lock().lines().next() {
                add_line_of_input(&mut vm, line.as_str());
            }
            println!();

            vm.run();
        } else {
            break;
        }
    }

    println!(
        "ENDING STATE: {:?}  CYCLES: {}",
        vm.get_state(),
        vm.get_cycles()
    );
    println!("OUTPUT: {}", vm.get_output());

    Ok(())
}
