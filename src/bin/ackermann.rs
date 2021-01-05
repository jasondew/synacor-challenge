use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

const MODULO: usize = 32768;

fn ackermann(m: usize, n: usize, k: usize, memo: &mut HashMap<(usize, usize), usize>) -> usize {
    if let Some(value) = memo.get(&(m, n)) {
        *value
    } else {
        let result = if m == 0 {
            (n + 1) % MODULO
        } else {
            if n == 0 {
                ackermann(m - 1, k, k, memo)
            } else {
                let x = ackermann(m, n - 1, k, memo);
                ackermann(m - 1, x, k, memo)
            }
        };
        memo.insert((m, n), result);
        result
    }
}

fn main() -> std::io::Result<()> {
    let m = 4;
    let n = 1;

    for k in 1..32768 {
        let mut memo: HashMap<(usize, usize), usize> = HashMap::new();

        if ackermann(m, n, k, &mut memo) == 6 {
            println!("\nk={}", k);
            break;
        } else {
            print!(".");
            io::stdout().flush().unwrap();
        }
    }

    Ok(())
}
