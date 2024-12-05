use lisp::Env;
use lisp::Evaluator;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::cell::RefCell;
use std::fs::OpenOptions;
use std::io;
use std::io::IsTerminal;
use std::io::Read;
use std::rc::Rc;

fn main() -> anyhow::Result<()> {
    let mut stdin = io::stdin();
    let mut env = Rc::new(RefCell::new(Env::new()));

    if !stdin.is_terminal() {
        let mut source = String::new();
        stdin.read_to_string(&mut source)?;

        Ok(println!("{:#?}", Evaluator::eval(source, &mut env)?))
    } else {
        repl(&mut env)
    }
}

fn repl(env: &mut Rc<RefCell<Env>>) -> anyhow::Result<()> {
    let mut rl = DefaultEditor::new()?;
    let mut history = dirs::home_dir().unwrap();
    history.push(".lisp_history");

    if rl.load_history(history.as_path()).is_err() {
        println!("REPL: No previous history");
        OpenOptions::new()
            .create(true)
            .write(true)
            .open(history.as_path())?;
    }

    let mut open_parens: i32 = 0;
    let mut buffer = String::new();

    loop {
        let readline = rl.readline(if open_parens > 0 { ".. " } else { "-> " });
        match readline {
            Ok(line) => {
                buffer.push_str(&line);
                buffer.push('\n');

                open_parens += line.chars().filter(|&c| c == '(').count() as i32;
                open_parens -= line.chars().filter(|&c| c == ')').count() as i32;

                match open_parens {
                    0 => {
                        rl.add_history_entry(buffer.as_str().trim())?;
                        println!("{:#?}", Evaluator::eval(buffer.as_str(), env)?);
                        buffer.clear();
                    }
                    ..0 => {
                        println!("REPL: BadRParen(s)");
                        open_parens = 0;
                        buffer.clear();
                    }
                    _ => (),
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("REPL: Error {}", err);
                break;
            }
        }
    }

    rl.save_history(history.as_path())?;
    Ok(())
}
