use std::fmt;

/// Defines some medium of which an Svg element can be represented
///
/// There are two implementation examples of this. On the web,
/// we use the leptos (web-sys inner) crate. This is faster and much safer
/// than embedding a string to be interpreted by the dom.
///
/// However, this cannot be performed in non-browser environments, like the server.
///
/// The server will instead use [`SvgEl`](super::SvgEl) to render the svg, which is
/// essentially a struct for representing xml.
///
/// Note that [`SvgEl`](super::SvgEl) will be replaced in the future by some general
/// XML struct in the future, but serves as a quick hack for productivity purposes.
pub trait SvgNode {
    fn set_attribute<S1, S2>(&mut self, key: S1, value: S2) -> &mut Self
    where
        S1: Into<String> + AsRef<str>,
        S2: fmt::Display;

    fn get_attribute<S>(&self, key: S) -> Option<String>
    where
        S: AsRef<str>;

    fn set_inner<S>(&mut self, inner: S) -> &mut Self
    where
        S: Into<String> + AsRef<str>;

    fn push_to_inner<S>(&mut self, to_add: S) -> &mut Self
    where
        S: Into<String> + AsRef<str>;

    fn prepend_child(&mut self, child: Self) -> &mut Self;

    fn append_child(&mut self, child: Self) -> &mut Self;

    fn outer_html(&self) -> String;

    fn svg_node() -> Self;

    fn path() -> Self;

    fn circle() -> Self;

    fn text() -> Self;

    fn defs() -> Self;

    fn linear_gradient() -> Self;

    fn stop() -> Self;
}
