use clap::{Parser, ValueEnum};
use std::fs;
use std::io::Read;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "rex")]
#[command(version = "0.1.0")]
#[command(about = "Rex - A coding language simpler than HTML, CSS, and JS")]
pub struct Args {
    #[arg(short, long)]
    pub file: Option<PathBuf>,

    #[arg(short, long, default_value = "index.html")]
    pub output: PathBuf,

    #[arg(short, long, value_enum, default_value = "html")]
    pub format: OutputFormat,

    #[arg(short, long)]
    pub watch: bool,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum OutputFormat {
    Html,
    Js,
    Css,
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let input = if let Some(path) = args.file {
        fs::read_to_string(&path)?
    } else {
        println!("Enter Rex code (Ctrl+C to exit):");
        let mut input = String::new();
        std::io::stdin().read_to_string(&mut input)?;
        input
    };

    let mut lexer = crate::lexer::Lexer::new(&input);
    let tokens = lexer.tokenize();
    println!("Tokens: {:?}", tokens);

    let parser = crate::parser::Parser::new();
    let program = parser.parse(tokens)?;
    println!("Parsed: {} statements", program.statements.len());

    let codegen = crate::codegen::CodeGen::new();
    let output = codegen.generate(&program)?;

    fs::write(&args.output, &output)?;
    println!("Output written to {:?}", args.output);

    Ok(())
}
