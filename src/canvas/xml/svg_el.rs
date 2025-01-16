use core::fmt;
use std::{borrow::Cow, io::Cursor};

use quick_xml::{events::BytesText, Writer};

use crate::canvas::SvgNode;

use super::XmlNode;

impl SvgNode for XmlNode<'_> {
    type Output = String;
    type Error = std::io::Error;

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

    fn push_child(&mut self, child: Self) -> &mut Self {
        XmlNode::push_child(self, child)
    }

    fn prepend_child(&mut self, child: Self) -> &mut Self {
        XmlNode::prepend_child(self, child)
    }

    fn push_text(&mut self, text: Cow<'_, str>) -> &mut Self {
        XmlNode::push_child(
            self,
            BytesText::from_escaped(quick_xml::escape::escape(text.into_owned())),
        )
    }

    fn outer_html(&self) -> String {
        self.to_string()
    }

    fn svg_node() -> Self {
        let mut node = Self::new("svg");

        node.push_attribute(("xmlns", "http://www.w3.org/2000/svg"));

        node
    }

    fn path() -> Self {
        Self::new("path")
    }

    fn circle() -> Self {
        Self::new("circle")
    }
    fn text() -> Self {
        Self::new("text")
    }
    fn defs() -> Self {
        Self::new("defs")
    }

    fn linear_gradient() -> Self {
        Self::new("linearGradient")
    }

    fn stop() -> Self {
        Self::new("stop")
    }

    fn build(self) -> Result<String, std::io::Error> {
        let mut writer: Writer<Cursor<Vec<u8>>> = Writer::new(Cursor::new(Vec::new()));

        self.write(&mut writer);

        let res = writer.into_inner().into_inner();

        Ok(String::from_utf8(res).unwrap_or_default())
    }
}
