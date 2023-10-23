use std::io::{self, BufRead, Write};

pub fn repl() {
    let lexerdef = spdx_lib::license_expression_lexer::lexerdef();
    let stdin = io::stdin();
    println!("Type SPDX license expressions (^D to exit).");
    loop {
        print!(">>> ");
        io::stdout().flush().ok();
        match stdin.lock().lines().next() {
            Some(Ok(ref l)) => {
                if l.trim().is_empty() {
                    continue;
                }
                let lexer = lexerdef.lexer(l);
                let (res, errs) = spdx_lib::license_expression_parser::parse(&lexer);
                for e in errs {
                    println!(
                        "{}",
                        e.pp(&lexer, &spdx_lib::license_expression_parser::token_epp)
                    );
                }
                match res {
                    Some(Ok(r)) => println!("Result: {:#?}", r),
                    _ => eprintln!("Unrecognizeable as an SPDX license expression."),
                }
            }
            _ => break,
        }
    }
}
