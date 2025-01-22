/*
TODO:
Adding cosmic-text v0.12.1 to dependencies
       Features:
       + fontconfig
       + rayon
       + std
       + swash
       + sys-locale
       - cosmic_undo_2
       - hashbrown
       - modit
       - monospace_fallback
       - no_std
       - shape-run-cache
       - syntect
       - vi
       - warn_on_missing_glyphs
       - wasm-web
*/

use cosmic_text::{Attrs, Buffer, FontSystem, Metrics, SwashCache};
use nalgebra::Scalar;

use crate::prelude::CommonConsts;

use super::TextStyle;

pub struct CosmicSystem {
    system: FontSystem,
    swash_cache: SwashCache,
}

impl CosmicSystem {
    pub fn write<U: Scalar + CommonConsts + Copy>(&mut self, style: &TextStyle<'_, '_, U>) {
        //font size and line height
        let metrics = Metrics::new(style.size().to_f32(), style.line_height().to_f32());

        let mut buffer = Buffer::new(&mut self.system, metrics);

        let mut buffer = buffer.borrow_with(&mut self.system);

        let bbox = style.bounding_box();

        buffer.set_size(Some(bbox.width().to_f32()), Some(bbox.height().to_f32()));

        let attrs = Attrs::new().family(*style.family());
    }
}
