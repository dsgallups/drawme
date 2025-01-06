use drawme::prelude::*;
fn main() {
    let circle = Circle::new((5.0, 5.0), 3.);

    let circle = circle.with_style(Fill::new(BLACK));
}
