use core::fmt;
use std::borrow::Cow;

use crate::canvas::SvgNode;

use super::XmlNode;

impl SvgNode for XmlNode<'_> {
    fn push_attribute<S1, S2>(&mut self, key: S1, value: S2) -> &mut Self
    where
        S1: Into<String> + AsRef<str>,
        S2: fmt::Display,
    {
        XmlNode::push_attribute(self, (key.as_ref(), Cow::Owned(value.to_string())))
    }

    fn get_attribute<S>(&self, key: S) -> Option<String>
    where
        S: AsRef<str>,
    {
        let v = key.as_ref();
        let v = v.as_bytes();
        match self.try_get_attribute(v) {
            Ok(attr) => attr.and_then(|a| a.unescape_value().ok().map(|a| a.clone().into_owned())),
            Err(_) => None,
        }
    }

    fn set_inner(&mut self, inner: Self) -> &mut Self {
        self.set_children(vec![inner])
    }

    fn push_to_inner(&mut self, to_add: Self) -> &mut Self {
        self.push_child(to_add)
    }

    fn prepend_child(&mut self, child: Self) -> &mut Self {
        XmlNode::prepend_child(self, child)
    }

    fn append_child(&mut self, child: Self) -> &mut Self {
        self.push_child(child)
    }

    fn outer_html(&self) -> String {
        self.to_string()
    }

    fn svg_node() -> Self {
        let mut node = Self::element("svg");

        node.push_attribute(("xmlns", "http://www.w3.org/2000/svg"));

        node
    }

    fn path() -> Self {
        Self::element("path")
    }

    fn circle() -> Self {
        Self::element("circle")
    }
    fn text() -> Self {
        Self::element("text")
    }
    fn defs() -> Self {
        Self::element("defs")
    }

    fn linear_gradient() -> Self {
        Self::element("linearGradient")
    }

    fn stop() -> Self {
        Self::element("stop")
    }
}
