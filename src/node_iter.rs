use html5ever::{Attribute, QualName};
use html5ever::tendril::StrTendril;
use markup5ever_rcdom::{Handle, RcDom};

pub trait AttrsExt {
    fn get(&self, name: &QualName) -> Option<StrTendril>;
    fn set(&mut self, name: &QualName, value: StrTendril);
}

impl AttrsExt for Vec<Attribute> {
    fn get(&self, name: &QualName) -> Option<StrTendril> {
        self.iter()
            .find(|attr| attr.name == *name)
            .map(|attr| attr.value.clone())
    }

    fn set(&mut self, name: &QualName, value: StrTendril) {
        let attr = self
            .iter_mut()
            .find(|attr| attr.name == *name);

        match attr {
            Some(attr) => {
                attr.value = value;
            }
            None => {
                self.push(Attribute {
                    name: name.clone(),
                    value,
                });
            }
        }
    }
}

pub trait NodeIter {
    fn find_first<F: FnMut(Handle) -> bool>(&self, predicate: F) -> Option<Handle>;
    fn for_each_node(&self, action: &mut impl FnMut(Handle));
}

impl NodeIter for RcDom {
    fn find_first<F: FnMut(Handle) -> bool>(&self, predicate: F) -> Option<Handle> {
        self.document.find_first(predicate)
    }
    fn for_each_node(&self, action: &mut impl FnMut(Handle)) {
        self.document.for_each_node(action)
    }
}

impl NodeIter for Handle {
    fn find_first<F: FnMut(Handle) -> bool>(&self, mut predicate: F) -> Option<Handle> {
        if predicate(self.clone()) {
            return Some(self.clone());
        }
        for child in self.children.borrow().iter().cloned() {
            if let Some(found) = child.find_first(&mut predicate) {
                return Some(found);
            }
        }
        None
    }
    fn for_each_node(&self, action: &mut impl FnMut(Handle)) {
        action(self.clone());
        for child in self.children.borrow().iter().cloned() {
            child.for_each_node(action);
        };
    }
}