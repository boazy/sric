use std::path::Path;
use anyhow::Context;
use html5ever::{parse_document, ParseOpts};
use html5ever::tendril::TendrilSink;
use html5ever::tree_builder::TreeBuilderOpts;
use markup5ever_rcdom::RcDom;

pub fn parse_html(filename: &Path) -> anyhow::Result<RcDom> {
    let opts = ParseOpts {
        tree_builder: TreeBuilderOpts {
            drop_doctype: true,
            ..Default::default()
        },
        ..Default::default()
    };

    let content = std::fs::read_to_string(filename)
        .with_context(|| format!("Failed to read file {:?}", filename))?;
    let dom = parse_document(RcDom::default(), opts)
        .from_utf8()
        .read_from(&mut content.as_bytes())
        .with_context(|| format!("Failed to parse HTML in file {:?}", filename))?;

    Ok(dom)
}