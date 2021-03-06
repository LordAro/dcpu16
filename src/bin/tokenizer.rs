extern crate dcpu16;
extern crate getopts;

mod cli;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use std::env;
use getopts::Options;
use dcpu16::assembler;
use std::process::exit;

fn main() {
    let mut opts = Options::new();
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    opts.optflag("c",
                 "canonize",
                 "resolves arithmetic literals before printing tokens");
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
        cli::print_usage(&program, "FILE", opts, &["-c program.asm"]);
        return;
    }

    if matches.opt_present("v") {
        cli::print_version(&program);
        return;
    }

    let canonize = matches.opt_present("c");

    if matches.free.len() != 1 {
        println!("Please input file");
        exit(1);
    }
    let ref filename = matches.free[0];
    let mut lines: Vec<String> = Vec::new();

    let x: &[_] = &[' ', '\n', '\t'];
    let path = Path::new(filename);
    let file = match File::open(&path) {
        Err(why) => {
            println!("Could load file {}: {}", path.display(), why);
            exit(1);
        }
        Ok(file) => file,
    };
    for line in BufReader::new(&file).lines() {
        match line {
            Ok(s) => lines.push(s.trim_matches(x).to_string()),
            Err(_) => {}
        }
    }
    let mut cpu = assembler::PCPU::new();
    let mut line_no = 0usize;
    for line in lines.iter() {
        let l = &line[..];

        if l.len() == 0 {
            line_no += 1;
            continue;
        }

        println!("\x1b[1;37m{}\x1b[0m", l);
        match assembler::tokenize(line_no, l, &mut cpu) {
            Ok(tokens) => {
                let new_tokens = if canonize {
                    assembler::canonize_tokens(&tokens)
                } else {
                    tokens.clone()
                };
                if new_tokens.len() > 0 {
                    for t in new_tokens.iter() {
                        println!("{} \x1b[1;30m{}:{}\x1b[0m", t.ttype, t.col, t.col + t.len);
                    }
                    println!("");
                }
            }
            Err(err) => {
                assembler::print_parse_error(&cpu, l, err);
            }
        };

        line_no += 1;
    }
}
