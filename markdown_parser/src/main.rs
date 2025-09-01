use clap::Parser;
use maud::{DOCTYPE, Markup, html};
use pulldown_cmark::{Options, Parser as MarkdownParser, html};
use std::{fs, path::PathBuf};

#[derive(Parser, Debug)]
struct Args {
    // for the full use --input or for the shorthand use the -i   -> cargo run -- --input Surya
    // input Markdown file path
    #[arg(long, short)]
    input: String,

    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn render_html_page(content: &str) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                title { "Markdown Page to HTML Outputs" }
            }
            body {
                (maud::PreEscaped(content.to_string()))
            }
        }
    }
}

fn main() {
    println!("Hii there");
    let args = Args::parse();
    println!("Arguments are as follow {:?}", args);
    let markdown_input = fs::read_to_string(&args.input).expect("Unable to read the Path");

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);

    let parser = MarkdownParser::new_ext(&markdown_input, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    let full_html_output = render_html_page(&html_output).into_string();
    match &args.output {
        Some(path) => fs::write(path, full_html_output).expect("Unable to write file"),
        None => println!(
            "path not Provided and content of the output is {}",
            full_html_output
        ),
    }
}
