use std::fs::File;
use std::path::PathBuf;
use anyhow::Context;
use structopt::StructOpt;
use crate::generate_sri::update_sri_for_dom;
use crate::write_html::write_html;

mod parse_html;
mod write_html;
mod generate_sri;
mod node_iter;
mod element;
mod mime_ext;
mod response_ext;

#[derive(Debug, StructOpt)]
#[structopt(name = "sric", about = "Automatically generate Subresource Integrity (SRI) hashes for HTML files.")]
struct Opts {
    /// Input file
    #[structopt(parse(from_os_str), name = "INPUT_FILE")]
    input: PathBuf,

    #[structopt(short, long, help = "Write the SRI hashes to the file in-place")]
    write: bool,

    #[structopt(parse(from_os_str), short, long, help="Output file", name = "OUTPUT_FILE")]
    output: Option<PathBuf>,

    #[structopt(short, long, help = "Override existing SRI hashes")]
    force: bool,
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::from_args();
    let mut dom = parse_html::parse_html(&opts.input)?;

    let output_filename = if opts.write {
        Some(opts.input.clone())
    } else {
        opts.output
    };

    update_sri_for_dom(&mut dom, opts.force);

    if let Some(output_filename) = &output_filename {
        let output = File::create(output_filename)
            .with_context(|| format!("Failed to create file {:?}", output_filename))?;
        write_html(output, dom)?;
    } else {
        write_html(std::io::stdout(), dom)?;
    };

    Ok(())
}

