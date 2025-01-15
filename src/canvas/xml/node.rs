use core::fmt;
use std::borrow::Cow;

use quick_xml::events::{
    attributes::{AttrError, Attribute},
    BytesStart, BytesText,
};

#[derive(Debug, Clone)]
pub enum XmlNode<'a> {
    Element {
        tag: BytesStart<'a>,
        inner: Option<Vec<XmlNode<'a>>>,
    },
    Text(BytesText<'a>),
}

impl fmt::Display for XmlNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<'a> XmlNode<'a> {
    pub fn element<C: Into<Cow<'a, str>>>(name: C) -> Self {
        Self::Element {
            tag: BytesStart::new(name),
            inner: None,
        }
    }

    pub fn is_element(&self) -> bool {
        matches!(self, XmlNode::Element { .. })
    }

    pub fn text<C: Into<Cow<'a, str>>>(text: C) -> Self {
        let bt = BytesText::from_escaped(quick_xml::escape::escape(text));
        Self::Text(bt)
    }

    pub fn set_children(&mut self, children: Vec<XmlNode<'a>>) -> &mut Self {
        if let XmlNode::Element { inner, .. } = self {
            *inner = Some(children);
        }
        self
    }

    pub fn with_children(mut self, children: Vec<XmlNode<'a>>) -> Self {
        self.set_children(children);
        self
    }

    pub fn push_child(&mut self, child: XmlNode<'a>) -> &mut Self {
        if let XmlNode::Element { inner, .. } = self {
            match inner {
                Some(children) => {
                    children.push(child);
                }
                None => {
                    *inner = Some(vec![child]);
                }
            }
        }
        self
    }

    /// Expensive
    pub fn prepend_child(&mut self, child: XmlNode<'a>) -> &mut Self {
        if let XmlNode::Element { inner, .. } = self {
            match inner {
                Some(children) => {
                    // adding one to capacity will sometimes be faster, and guarantees
                    // that memory is only copied once
                    let mut new_vec = Vec::with_capacity(children.capacity() + 1);
                    new_vec.push(child);
                    new_vec.extend(children.clone());
                    *children = new_vec;
                }
                None => {
                    *inner = Some(vec![child]);
                }
            }
        }
        self
    }

    /// Clears the original tag and attributes
    pub fn new_tag_name<C: Into<Cow<'a, str>>>(&mut self, name: C) -> &mut Self {
        self.new_tag(BytesStart::new(name))
    }
    pub fn new_tag(&mut self, tag: BytesStart<'a>) -> &mut Self {
        if let XmlNode::Element { tag: st, .. } = self {
            *st = tag;
        }
        self
    }

    pub fn try_get_attribute<N: AsRef<[u8]> + Sized>(
        &'a self,
        name: N,
    ) -> Result<Option<Attribute<'a>>, AttrError> {
        if let XmlNode::Element { tag, .. } = self {
            return tag.try_get_attribute(name);
        } else {
            //todo: this is an error
            return Ok(None);
        }
    }

    /// Consumes `self` and yield a new `BytesStart` with additional attributes from an iterator.
    ///
    /// The yielded items must be convertible to [`Attribute`] using `Into`.
    pub fn with_attributes<'b, I>(mut self, attributes: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<Attribute<'b>>,
    {
        self.extend_attributes(attributes);
        self
    }
    pub fn push_attribute<'b, A>(&mut self, attribute: A) -> &mut Self
    where
        A: Into<Attribute<'b>>,
    {
        if let XmlNode::Element { tag, .. } = self {
            tag.push_attribute(attribute);
        }
        self
    }

    /// Add additional attributes to this tag using an iterator.
    ///
    /// The yielded items must be convertible to [`Attribute`] using `Into`.
    pub fn extend_attributes<'b, I>(&mut self, attributes: I) -> &mut Self
    where
        I: IntoIterator,
        I::Item: Into<Attribute<'b>>,
    {
        if let XmlNode::Element { tag, .. } = self {
            tag.extend_attributes(attributes);
        }
        self
    }
}
