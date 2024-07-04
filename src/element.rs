use std::cell::RefCell;
use html5ever::{Attribute, QualName};
use markup5ever_rcdom::{Handle, NodeData};

pub struct Element {
    pub name: QualName,
    pub attrs: Vec<Attribute>,
    template_contents: Option<Handle>,
    mathml_annotation_xml_integration_point: bool,
}
impl From<Element> for NodeData {
    fn from(element: Element) -> NodeData {
        NodeData::Element {
            name: element.name,
            attrs: RefCell::new(element.attrs),
            template_contents: RefCell::new(element.template_contents),
            mathml_annotation_xml_integration_point: element.mathml_annotation_xml_integration_point,
        }
    }
}

impl TryFrom<&NodeData> for Element {
    type Error = &'static str;

    fn try_from(value: &NodeData) -> Result<Self, Self::Error> {
        match value {
            NodeData::Element {
                name,
                attrs,
                template_contents,
                mathml_annotation_xml_integration_point,
            } => Ok(Element {
                name: name.clone(),
                attrs: attrs.clone().into_inner(),
                template_contents: template_contents.clone().into_inner(),
                mathml_annotation_xml_integration_point: mathml_annotation_xml_integration_point.clone(),
            }),
            _ => Err("NodeData is not an Element")
        }
    }
}