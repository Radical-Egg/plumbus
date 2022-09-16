use clipboard::{ClipboardContext, ClipboardProvider};
use std::io::{self, BufRead};

fn main() {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    let stdin = io::stdin();
    let mut lines = stdin
        .lock()
        .lines()
        .fold(String::new(), |accum, line| accum + &line.unwrap() + "\n");
    lines.pop();

    println!("{}", &lines);

    ctx.set_contents(lines).unwrap();
    ctx.get_contents().unwrap();
}
