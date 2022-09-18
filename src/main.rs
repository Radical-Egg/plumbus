use atty::Stream;
use clap::Parser;
use plumbus;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(name = "Plumbus")]
#[clap(author = "R.Egg <egg95@protonmail.com>")]
#[clap(version = "1.0.3")]
#[clap(about = "Copy the output of a command to your clipboard. 
    Use the --file option to copy the contents of a file", long_about = None)]
struct Args {
    /// Copy the contents of a file
    /// plumbus --file hello.txt
    #[clap(short, long, value_parser, verbatim_doc_comment, default_value_t)]
    file: String,
}
fn main() {
    let args = Args::parse();

    if atty::is(Stream::Stdin) {
        plumbus::copy_file_clipboard(args.file);
        return;
    }
    let stdin = std::io::stdin();
    plumbus::copy_to_clipboard(stdin);
}
