use anyhow::Context;
use clap::Parser;
use clio::{InputPath, OutputPath};

use crate::generate_sri::update_sri_for_dom;
use crate::write_html::write_html;

mod parse_html;
mod write_html;
mod generate_sri;
mod node_iter;
mod element;
mod mime_ext;
mod response_ext;

#[derive(Debug, Parser)]
#[command(version, name = "sric", about = "Automatically generate Subresource Integrity (SRI) hashes for HTML files.")]
struct Opts {
    /// Input file
    #[arg(value_parser, name = "INPUT_FILE")]
    input: InputPath,

    #[arg(short, long, group = "output", help = "Write the SRI hashes to the file in-place")]
    write: bool,

    #[arg(value_parser, short, long, default_value = "-", group = "output", help="Output file", name = "OUTPUT_FILE")]
    output: OutputPath,

    #[arg(short, long, help = "Override existing SRI hashes")]
    force: bool,
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    let mut dom = parse_html::parse_html(opts.input.clone())?;

    let output = if opts.write {
        let input = opts.input;
        if input.is_std() {
            OutputPath::std()
        } else {
            let path = input.path().as_os_str();
            OutputPath::new(path)?
        }
    } else {
        opts.output
    };

    update_sri_for_dom(&mut dom, opts.force);

    let output_file = output.clone().create()
        .with_context(|| format!("Failed to create file {}", output))?;
    write_html(output_file, dom)?;

    Ok(())
}

