extern crate dcpu16;
extern crate getopts;

mod cli;

use std::env;
use std::fs::File;
use std::io::Read;
use std::error::Error;
use std::path::Path;
use getopts::Options;
use dcpu16::dcpu;
use dcpu16::disassembler;
use std::process::exit;

fn main() {
    let mut opts = Options::new();
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    opts.optflag("m", "no-color", "do not use ANSI colors in output");
    opts.optflag("v", "version", "print version");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(why) => {
            println!("{}", why);
            exit(1);
        }
    };

    if matches.opt_present("h") {
        cli::print_usage(&program, "FILE", opts, &["program.bin"]);
        return;
    }

    if matches.opt_present("v") {
        cli::print_version(&program);
        return;
    }

    if matches.free.len() != 1 {
        println!("Please input file");
        return;
    }
    let color = !matches.opt_present("m");
    let ref filename = matches.free[0];

    let mut cpu = dcpu::DCPU::new();

    let path = Path::new(filename);
    let mut file = match File::open(&path) {
        Err(why) => {
            println!("Could not open file {}: {}",
                     path.display(),
                     why.description());
            exit(1);
        }
        Ok(f) => f,
    };

    let mut nwords = 0usize;
    let mut buffer: Vec<u8> = Vec::new();
    let res = file.read_to_end(&mut buffer);
    match res {
        Ok(_) => {
            let mut j = 0;
            let mut sig = 0u16;
            for v in buffer {
                if j % 2 == 0 {
                    sig = v as u16;
                } else {
                    cpu.mem[nwords] = (sig << 8) + (v as u16);
                    nwords += 1;
                }

                j += 1;
            }
        }
        Err(why) => {
            println!("Could not read contents of file: {}", why);
            exit(1);
        }
    }

    loop {
        if cpu.pc as usize >= nwords {
            break;
        }
        let (offset, s) = disassembler::disassemble_instruction(&cpu, color);
        cpu.pc += offset;
        println!("{}", s);
    }
}
