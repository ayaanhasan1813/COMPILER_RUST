extern crate rcompiler;

use rcompiler::gen_ir::gen_ir;
use rcompiler::gen_x86::gen_x86;
use rcompiler::irdump::dump_ir;
use rcompiler::parse::parse;
use rcompiler::preprocess::Preprocessor;
use rcompiler::regalloc::alloc_regs;
use rcompiler::sema::sema;
use rcompiler::token::tokenize;

use std::env;
use std::process;

fn usage() -> ! {
    eprintln!("Usage: 9cc [-dump-ir1] [-dump-ir2] <file>");
    process::exit(1)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        usage();
    }

    let mut dump_ir1 = false;
    let mut dump_ir2 = false;
    let path;

    if args.len() == 3 && args[1] == "-dump-ir1" {
        dump_ir1 = true;
        path = args[2].clone();
    } else if args.len() == 3 && args[1] == "-dump-ir2" {
        dump_ir2 = true;
        path = args[2].clone();
    } else {
        if args.len() != 2 {
            usage();
        }
        path = args[1].clone();
    }

    // Tokenize and parse.
    let tokens = tokenize(path, &mut Preprocessor::new());

    let nodes = parse(&tokens);
    let (nodes, globals) = sema(nodes);
    let mut fns = gen_ir(nodes);

    if dump_ir1 {
        dump_ir(&fns);
    }

    alloc_regs(&mut fns);

    if dump_ir2 {
        dump_ir(&fns);
    }

    gen_x86(globals, fns);
}
