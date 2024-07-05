use anyhow::Context;
use clio::InputPath;
use html5ever::{parse_document, ParseOpts};
use html5ever::tendril::TendrilSink;
use html5ever::tree_builder::TreeBuilderOpts;
use markup5ever_rcdom::RcDom;

pub fn parse_html(input_path: InputPath) -> anyhow::Result<RcDom> {
    let opts = ParseOpts {
        tree_builder: TreeBuilderOpts {
            drop_doctype: true,
            ..Default::default()
        },
        ..Default::default()
    };

    let input_file = input_path.clone().open()
        .with_context(|| format!("Failed to open file {input_path}"))?;
    let content = std::io::read_to_string(input_file)
        .with_context(|| format!("Failed to read file {input_path}"))?;

    let dom = parse_document(RcDom::default(), opts)
        .from_utf8()
        .read_from(&mut content.as_bytes())
        .with_context(|| format!("Failed to parse HTML in file {input_path}"))?;

    Ok(dom)
}