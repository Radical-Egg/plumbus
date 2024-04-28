extern crate copypasta;

use copypasta::{ClipboardContext, ClipboardProvider};
use std::io::{self, BufRead};

pub fn copy_to_clipboard(stdin: io::Stdin) -> Result<String, Box<dyn std::error::Error>>  {
    let mut ctx: ClipboardContext = ClipboardContext::new().unwrap();
    let mut lines = stdin
        .lock()
        .lines()
        .fold(String::new(), 
            |accum, line| accum + &line.unwrap() + "\n");
    lines.pop();

    ctx.set_contents(lines).unwrap();
    let content = ctx.get_contents().unwrap();
    Ok(content)
}

pub fn copy_file_clipboard(fp: String) -> Result<(), Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(fp)?;

    let mut ctx: ClipboardContext = ClipboardContext::new().unwrap();

    ctx.set_contents(contents).unwrap();
    ctx.get_contents().unwrap();

    Ok(())
}
