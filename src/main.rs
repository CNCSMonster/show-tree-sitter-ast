use clap::ValueEnum;
use std::{fs, path::Path};
use tree_sitter::Parser;

#[derive(Debug, clap::Parser)]
pub struct Cli {
    #[arg(short, long)]
    file: String,
    #[arg(short, long)]
    language: Language,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Language {
    Cpp,
    Rust,
}

fn main() -> anyhow::Result<()> {
    use clap::Parser;
    let cli = Cli::parse();
    // 初始化 Tree-sitter 解析器
    let mut parser = tree_sitter::Parser::new();

    let lang: tree_sitter::Language = match cli.language {
        Language::Cpp => tree_sitter_cpp::LANGUAGE.into(),
        Language::Rust => tree_sitter_rust::LANGUAGE.into(),
    };
    parser.set_language(&lang)?;
    show_ast(&mut parser, cli.file.as_ref())
}

pub fn show_ast(parser: &mut Parser, file: &Path) -> anyhow::Result<()> {
    let source_code = fs::read_to_string(file).expect("Unable to read file");

    let tree = parser.parse(&source_code, None).unwrap();

    println!("{}", &tree.root_node().to_sexp());
    Ok(())
}
