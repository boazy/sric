use std::str::FromStr;

use anyhow::{bail, Context};
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use html5ever::{Attribute, local_name, namespace_url, ns, QualName};
use html5ever::tendril::StrTendril;
use lazy_static::lazy_static;
use markup5ever_rcdom::{Handle, NodeData, RcDom};
use mime::Mime;
use sha2::Digest;
use url::Url;

use crate::mime_ext::MimeExt;
use crate::node_iter::{AttrsExt, NodeIter};
use crate::response_ext::ResponseExt;

lazy_static!(
    static ref QNAME_SCRIPT: QualName = QualName::new(None, ns!(html), local_name!("script"));
    static ref QNAME_LINK: QualName = QualName::new(None, ns!(html), local_name!("link"));
    static ref QNAME_INTEGRITY: QualName = QualName::new(None, ns!(), local_name!("integrity"));
    static ref QNAME_SRC: QualName = QualName::new(None, ns!(), local_name!("src"));
    static ref QNAME_HREF: QualName = QualName::new(None, ns!(), local_name!("href"));
    static ref QNAME_REL: QualName = QualName::new(None, ns!(), local_name!("rel"));
);

pub fn update_sri_for_dom(dom: &mut RcDom, force: bool) {
    dom.for_each_node(&mut |node: Handle| {
        let node_data = &node.data;
        add_sri(&node_data, force);
    });
}

fn add_sri(node_data: &NodeData, force: bool) {
    match node_data {
        NodeData::Element { ref name, ref attrs, .. } => {
            add_sri_to_element(name, &mut attrs.borrow_mut(), force)
        }
        _ => {}
    }
}

fn add_sri_to_element(name: &QualName, attrs: &mut Vec<Attribute>, force: bool) {
    if let Some(_) = attrs.get(&QNAME_INTEGRITY) {
        // Override existing SRI hash if force is enabled
        if force {
            if let Some(sri) = generate_sri(name, attrs) {
                attrs.set(&QNAME_INTEGRITY, sri);
            };
        }
    } else if let Some(sri) = generate_sri(name, attrs) {
        attrs.push(Attribute {
            name: QNAME_INTEGRITY.clone(),
            value: sri,
        });
    }
}

fn generate_sri(name: &QualName, attrs: &Vec<Attribute>) -> Option<StrTendril> {
    let src =
        if name.eq(&QNAME_SCRIPT) {
            attrs.get(&QNAME_SRC)
        } else if name.eq(&QNAME_LINK) && is_rel_with_sri(attrs) {
            attrs.get(&QNAME_HREF)
        } else {
            None
        };
    let Some(src) = src else { return None };

    match get_sri_from_url(&src) {
        Ok(sri) => Some(StrTendril::from(sri)),
        Err(err) => {
            eprintln!("Failed to generate SRI for {}: {}", src, err);
            None
        }
    }
}

fn get_sri_from_url(url: &str) -> anyhow::Result<String> {
    let response = ureq::get(url)
        .call()
        .context("Failed to read integrity hash from url")?;

    let url = Url::parse(url)
        .context("Failed to parse url")?;

    let content_type = response.content_type();
    let content_type = Mime::from_str(content_type)
        .with_context(|| format!("Failed to parse content type: {content_type}"))?;
    let mime_types = mime_guess::from_path(url.path());
    if mime_types.iter().any(|mime| mime.is_same_essence(&content_type)) {
        let bytes = response.read_bytes_with_limit(10 * 1024 * 1024)?;
        let sri = BASE64_STANDARD.encode(&sha2::Sha384::digest(bytes));
        Ok(format!("sha384-{}", sri))
    } else {
        bail!("Invalid content type for extension: {content_type}")
    }
}

fn is_rel_with_sri(attrs: &Vec<Attribute>) -> bool {
    let Some(rel) = attrs.get(&QNAME_REL) else { return false };
    let rel: &[u8] = rel.as_bytes();
    rel == b"stylesheet" || rel == b"preload" || rel == b"modulepreload"
}