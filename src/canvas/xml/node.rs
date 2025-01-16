use core::fmt;
use std::{borrow::Cow, io::Write};

use quick_xml::{
    events::{
        attributes::{AttrError, Attribute},
        BytesStart, BytesText, Event,
    },
    Writer,
};

#[derive(Debug, Clone)]
pub enum XmlChild<'a> {
    Element(XmlNode<'a>),
    Text(BytesText<'a>),
}

impl<'a> XmlChild<'a> {
    pub fn is_element(&self) -> bool {
        matches!(self, XmlChild::Element { .. })
    }

    pub fn text<C: Into<Cow<'a, str>>>(text: C) -> Self {
        let bt = BytesText::from_escaped(quick_xml::escape::escape(text));
        Self::Text(bt)
    }

    pub fn write<W: Write>(self, writer: &mut Writer<W>) -> std::io::Result<()> {
        match self {
            XmlChild::Text(t) => writer.write_event(Event::Text(t)),
            XmlChild::Element(e) => e.write(writer),
        }
    }
}

impl<'a> From<XmlNode<'a>> for XmlChild<'a> {
    fn from(value: XmlNode<'a>) -> Self {
        Self::Element(value)
    }
}

impl<'a> From<BytesText<'a>> for XmlChild<'a> {
    fn from(value: BytesText<'a>) -> Self {
        Self::Text(value)
    }
}

trait BytesStartExt<'a> {
    fn into_event(self) -> Event<'a>;
}

impl<'a> BytesStartExt<'a> for BytesStart<'a> {
    fn into_event(self) -> Event<'a> {
        Event::Start(self)
    }
}

#[derive(Debug, Clone)]
pub struct XmlNode<'a> {
    tag: BytesStart<'a>,
    inner: Option<Vec<XmlChild<'a>>>,
}

impl fmt::Display for XmlNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<'a> XmlNode<'a> {
    pub fn new<C: Into<Cow<'a, str>>>(name: C) -> Self {
        Self {
            tag: BytesStart::new(name),
            inner: None,
        }
    }

    pub fn set_children<I>(&mut self, children: I) -> &mut Self
    where
        I: IntoIterator,
        I::Item: Into<XmlChild<'a>>,
    {
        self.inner = Some(children.into_iter().map(|c| c.into()).collect());
        self
    }

    pub fn with_children<I>(mut self, children: Vec<XmlChild<'a>>) -> Self
    where
        I: IntoIterator,
        I::Item: Into<XmlChild<'a>>,
    {
        self.set_children(children);
        self
    }

    pub fn push_child(&mut self, child: impl Into<XmlChild<'a>>) -> &mut Self {
        match &mut self.inner {
            Some(children) => {
                children.push(child.into());
            }
            None => {
                self.inner = Some(vec![child.into()]);
            }
        }
        self
    }

    /// Expensive
    pub fn prepend_child(&mut self, child: impl Into<XmlChild<'a>>) -> &mut Self {
        match &mut self.inner {
            Some(children) => {
                // adding one to capacity will sometimes be faster, and guarantees
                // that memory is only copied once
                let mut new_vec = Vec::with_capacity(children.capacity() + 1);
                new_vec.push(child.into());
                new_vec.extend(children.clone());
                *children = new_vec;
            }
            None => {
                self.inner = Some(vec![child.into()]);
            }
        }
        self
    }

    /// Clears the original tag and attributes
    pub fn set_tag_name<C: Into<Cow<'a, str>>>(&mut self, name: C) -> &mut Self {
        self.set_tag(BytesStart::new(name))
    }
    pub fn set_tag(&mut self, tag: BytesStart<'a>) -> &mut Self {
        self.tag = tag;
        self
    }

    pub fn try_get_attribute<N: AsRef<[u8]> + Sized>(
        &'a self,
        name: N,
    ) -> Result<Option<Attribute<'a>>, AttrError> {
        self.tag.try_get_attribute(name)
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
        self.tag.push_attribute(attribute);
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
        self.tag.extend_attributes(attributes);
        self
    }

    pub fn write<W: Write>(self, writer: &mut Writer<W>) -> std::io::Result<()> {
        let Some(children) = self.inner else {
            return writer.write_event(Event::Empty(self.tag));
        };

        writer.write_event(Event::Start(self.tag.borrow()))?;
        for child in children {
            child.write(writer)?;
        }
        writer.write_event(Event::End(self.tag.to_end()))?;

        Ok(())
    }
}
