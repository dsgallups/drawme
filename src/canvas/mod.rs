use std::sync::{Arc, LazyLock};

use fontdb::Database;
use nalgebra::{Point2, Scalar};

use crate::prelude::*;

#[cfg(feature = "debug")]
pub mod debug;
#[cfg(feature = "svg")]
pub mod svg;
#[cfg(feature = "xml")]
pub mod xml;

pub static FONT_DATABASE: LazyLock<Arc<Database>> = LazyLock::new(|| {
    let mut db = Database::new();

    db.load_system_fonts();
    db.set_serif_family("Times New Roman");
    db.set_sans_serif_family("Arial");
    db.set_cursive_family("Comic Sans MS");
    db.set_fantasy_family("Impact");
    db.set_monospace_family("Courier New");

    Arc::new(db)
});

pub trait Canvas {
    type Unit: DrawUnit;

    fn path<S: AsDrawStyle<Unit = Self::Unit>>(&mut self, style: S, path: &Path<Self::Unit>);
    /*fn text<S: AsDrawStyle<Unit = Self::Unit>>(
        &mut self,
        style: S,
        text: &str,
        font: &FontProps<'_>,
        isometry: Isometry,
    );*/
    fn text(&mut self, text: &str, style: &TextStyle<'_, '_, Self::Unit>) {
        let query = style.to_query();
        let Some(id) = FONT_DATABASE.query(&query) else {
            return;
        };
    }

    fn rectangle<S: AsDrawStyle<Unit = Self::Unit>>(
        &mut self,
        style: S,
        rectangle: &Rectangle<Self::Unit>,
    ) {
        let top_left = rectangle.top_left();
        let bottom_right = rectangle.bottom_left();
        let mut path = Path::with_capacity(5);
        path.move_to(top_left);
        path.line_to(Point2::new(bottom_right.x, top_left.y));
        path.line_to(bottom_right);
        path.line_to(Point2::new(top_left.x, bottom_right.y));
        path.line_to(top_left);

        self.path(style, &path);
    }
    fn circle<S: AsDrawStyle<Unit = Self::Unit>>(
        &mut self,
        style: S,
        point: Point2<Self::Unit>,
        radius: Self::Unit,
    );
    fn image(&mut self, src: &ImageSource);
}
