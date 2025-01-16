use drawme::prelude::*;

#[test]
fn simple_circle() {
    // Create a circle with a radius of 3 at position x: 5, y: 5
    let circle = Circle::new((5., 5.), 3.);
    let svg: XmlSvg = circle.with_style(Fill::new(BLACK)).draw_onto_canvas();

    assert_eq!(
        svg.build().unwrap(),
        r#"<svg xmlns="http://w3.org/2000/svg" viewBox="0 0 8 8"><circle cx="5" cy="5" r="3" fill="rgb(0, 0, 0)"/></svg>"#.as_bytes()
    );
}
