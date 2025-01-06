#![doc = r#"

## Why not use a color library?
This crate *should* use a color library, but the
following do not work:

- `bevy_color`: does not implement Serialize
"#]

mod paint;
pub use paint::*;

mod gradient;
pub use gradient::*;

mod solid;
pub use solid::*;

mod rgb;
pub use rgb::*;

mod rgba;
pub use rgba::*;
