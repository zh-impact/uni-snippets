use clap::Parser;
use pdf_extract_cli::adapter::ad_pdf_extract;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,
}

fn main() {
    let args = Args::parse();

    if args.input.is_empty() {
        eprintln!("Please provide a PDF file path using --input or -i");
        std::process::exit(1);
    }

    match ad_pdf_extract::extract(args.input) {
        Ok(text) => println!("{}", text),
        Err(e) => eprintln!("Error: {}", e),
    }
}
