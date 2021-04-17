use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::stdin;
use std::num::Wrapping;
 
fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Uso: {} [caminho] (--debug)", args[0]);
        return;
    }
 
    let src: Vec<char> = {
        let mut buf = String::new();
        match File::open(&args[1])
        {
            Ok(mut f) => { f.read_to_string(&mut buf).unwrap(); }
            Err(e)    => {
                println!("Erro ao abrir o arquivo '{}': {}", args[1], e);
                return;
            }
        }
 
        buf.chars().collect()
    };
 
    let brackets: HashMap<usize, usize> = {
        let mut m = HashMap::new();
        let mut scope_stack = Vec::new();
        for (idx, ch) in src.iter().enumerate() {
            match ch {
                &'[' => { scope_stack.push(idx); }
                &']' => { m.insert(scope_stack.pop().unwrap(), idx); }
                _    => { /* ignorar */ }
            }
        }
 
        m
    };
 
    let mut pc: usize = 0;                                  // Contador de programa
    let mut mem: [Wrapping<u8>;5000] = [Wrapping(0);5000];  
    let mut ptr: usize = 0;                                 // Ponteiro
    let mut stack: Vec<usize> = Vec::new();                 // Vetor
 
    let stdin_ = stdin();
    let mut reader = stdin_.lock().bytes();
    while pc < src.len() {
        let Wrapping(val) = mem[ptr];
 
       
        const ONE: Wrapping<u8> = Wrapping(1);
        match src[pc] {
            '>' => { ptr += 1; }
            '<' => { ptr -= 1; }
 
            '+' => { mem[ptr] = mem[ptr] + ONE; }
            '-' => { mem[ptr] = mem[ptr] - ONE; }
 
            '[' => {
                if val == 0 {
                    pc = brackets[&pc];
                } else {
                    stack.push(pc);
                }
            }
            ']' => {
                let matching_bracket = stack.pop().unwrap();
                if val != 0 {
                    pc = matching_bracket - 1;
                }
            }
 
            '.' => {
               
                    print!("{}", val as char);
                
            }
            ',' => {
                mem[ptr] = Wrapping(reader.next().unwrap().unwrap());
            }
 
            _   => { /* ignorar */ }
        }
 
        pc += 1;
    }
}