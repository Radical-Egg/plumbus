use clipboard::{ClipboardContext, ClipboardProvider};
use std::io::{self, BufRead};
use std::panic;

pub fn copy_to_clipboard(stdin: io::Stdin) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let mut lines = stdin
        .lock()
        .lines()
        .fold(String::new(), |accum, line| accum + &line.unwrap() + "\n");
    lines.pop();

    ctx.set_contents(lines).unwrap();
    ctx.get_contents().unwrap();
}

pub fn copy_file_clipboard(fp: String) {
    let contents = std::fs::read_to_string(fp);

    match contents {
        Ok(content) => {
            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
            ctx.set_contents(content).unwrap();
            ctx.get_contents().unwrap();
        }
        Err(err) => {
            panic::set_hook(Box::new(|info| {
                println!("{}", info);
            }));
            panic!("{}", err);
        }
    };
}
