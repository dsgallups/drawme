use std::borrow::Cow;

use quick_xml::events::{attributes::Attribute, BytesStart, BytesText};

enum XmlNodeInner<'a> {
    Nodes(Vec<XmlNode<'a>>),
    Text(BytesText<'a>),
}

pub struct XmlNode<'a> {
    tag: BytesStart<'a>,
    inner: Option<XmlNodeInner<'a>>,
}

impl<'a> XmlNode<'a> {
    pub fn builder<C: Into<Cow<'a, str>>>(tag_name: C) -> XmlNodeBuilder<'a> {
        XmlNodeBuilder::new(tag_name)
    }
}

pub struct XmlNodeBuilder<'a> {
    tag: BytesStart<'a>,
}

impl<'a> XmlNodeBuilder<'a> {
    pub fn new<C: Into<Cow<'a, str>>>(name: C) -> Self {
        Self {
            tag: BytesStart::new(name),
        }
    }

    /// Clears the original tag and attributes
    pub fn new_tag_name<C: Into<Cow<'a, str>>>(self, name: C) -> Self {
        self.new_tag(BytesStart::new(name))
    }
    pub fn new_tag(mut self, tag: BytesStart<'a>) -> Self {
        self.tag = tag;
        self
    }

    /// Consumes `self` and yield a new `BytesStart` with additional attributes from an iterator.
    ///
    /// The yielded items must be convertible to [`Attribute`] using `Into`.
    pub fn with_attributes<'b, I>(mut self, attributes: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<Attribute<'b>>,
    {
        self.tag = self.tag.with_attributes(attributes);
        self
    }

    /// Add additional attributes to this tag using an iterator.
    ///
    /// The yielded items must be convertible to [`Attribute`] using `Into`.
    pub fn extend_attributes<'b, I>(mut self, attributes: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<Attribute<'b>>,
    {
        self.tag.extend_attributes(attributes);
        self
    }

    pub fn build(self) -> XmlNode<'a> {
        XmlNode {
            tag: self.tag,
            inner: None,
        }
    }
}
