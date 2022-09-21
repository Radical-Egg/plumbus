use atty::Stream;
use clap::Parser;
use plumbus;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(name = "Plumbus")]
#[clap(author = "R.Egg <egg95@protonmail.com>")]
#[clap(version = "1.0.5")]
#[clap(about = "Copy the output of a command to your clipboard. 
Use the --file option to copy the contents of a file

usage: echo 'hello' | plumbus ", long_about = None)]
struct Args {
    /// Copy the contents of a file
    /// plumbus -f hello.txt
    #[clap(short, long, value_parser, verbatim_doc_comment, default_value_t)]
    file: String,

    /// Use the verbose flag to show output
    /// echo "hello world!" | plumbus -v
    #[clap(short, long, verbatim_doc_comment)]
    verbose: bool,
}
fn main() {
    let args = Args::parse();

    if atty::is(Stream::Stdin) {
        if args.file != String::new() {
            match plumbus::copy_file_clipboard(args.file).err() {
                Some(err) => {
                    println!("{}", err.to_string());
                    return
                },
                None => return
            }
        }
        return;
    }

    let stdin = std::io::stdin();

    match plumbus::copy_to_clipboard(stdin) {
        Ok(content) => {
            if args.verbose {
                println!("{}", content);
            } 
        },
        Err(err) => {
            println!("{}", err.to_string());
        }
    }
}
