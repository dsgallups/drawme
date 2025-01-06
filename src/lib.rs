#![doc = r#"
Draw anything, to anything


# Overview
`drawme` contains interfaces and types that enable developers to draw their types to
static, two-dimensional [`Canvas`]es.

Much of the design relies on SVG's specification, meaning that
it is up to the [`Canvas`] on how to handle the units.

If you want to scale, contract, clip, etc. the drawing does not care.


## TODO
unit will always be f64,
it is up to the canvas on how to scale accordingly,
to define a bounding box, etc.

Drawings can calculate their total size
"#]

pub mod placement;
pub mod shapes;

pub mod prelude {
    pub use crate::placement::*;
    pub use crate::shapes::*;
}
