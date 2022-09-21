use clipboard::{ClipboardContext, ClipboardProvider};
use std::io::{self, BufRead};

pub fn copy_to_clipboard(stdin: io::Stdin) -> Result<String, Box<dyn std::error::Error>>  {
    let mut ctx: ClipboardContext = ClipboardProvider::new()?;
    let mut lines = stdin
        .lock()
        .lines()
        .fold(String::new(), 
            |accum, line| accum + &line.unwrap() + "\n");
    lines.pop();

    ctx.set_contents(lines)?;
    let content = ctx.get_contents()?;
    Ok(content)
}

pub fn copy_file_clipboard(fp: String) -> Result<(), Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(fp)?;

    let mut ctx: ClipboardContext = ClipboardProvider::new()?;

    ctx.set_contents(contents)?;
    ctx.get_contents()?;

    Ok(())
}
