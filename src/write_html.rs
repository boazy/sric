use std::io::Write;
use anyhow::Context;
use html5ever::serialize;
use markup5ever_rcdom::{RcDom, SerializableHandle};

pub fn write_html<W: Write>(mut writer: W, dom: RcDom) -> anyhow::Result<()> {
    writer.write(b"<!DOCTYPE html>\n")?;
    let document: SerializableHandle = dom.document.into();
    serialize(writer, &document, Default::default())
        .context("Failed to serialize HTML document")?;
    Ok(())
}
