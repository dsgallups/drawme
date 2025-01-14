use debug::Dbg;
use drawme::prelude::*;

fn main() {
    // Create a circle with a radius of 3 at position x: 5, y: 5
    let circle = Circle::new((5., 5.), 3.);
    let svg: Dbg<Svg> = circle.with_style(Fill::new(BLACK)).draw_onto_canvas();

    panic!("{:?}", svg);

    //svg will have a width and height of 8px.
}
