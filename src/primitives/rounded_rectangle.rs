/// RoundedRectangle is a rectangle with rounded corners
///
/// See [`Rectangle`] for more information on undocumented methods.
#[derive(Debug, Clone)]
pub struct RoundedRectangle<Unit = f64> {
    inner: Rectangle<Unit>,
    /// The radius of the top-left corner
    pub top_left_radius: Unit,
    /// The radius of the top-right corner
    pub top_right_radius: Unit,
    /// The radius of the bottom-left corner
    pub bottom_left_radius: Unit,
    /// The radius of the bottom-right corner
    pub bottom_right_radius: Unit,
}

impl<Unit> RoundedRectangle<Unit> {
    pub fn set_all_corners(&mut self, radius: Unit) -> &mut Self
    where
        Unit: Clone,
    {
        self.top_left_radius = radius.clone();
        self.top_right_radius = radius.clone();
        self.bottom_left_radius = radius.clone();
        self.bottom_right_radius = radius;

        self
    }

    pub fn into_inner_rectangle(self) -> Rectangle<Unit> {
        self.inner
    }

    /// Rotates around its center. RoundedRectangle does not implement transformation since it cannot be rotated around a point.
    pub fn rotate(&mut self, rot: Rotation) {
        self.inner.rotate(rot);
    }
}

impl<Unit: Zero> RoundedRectangle<Unit> {
    pub fn from_rectangle(rect: Rectangle<Unit>) -> Self {
        Self {
            inner: rect,
            top_left_radius: Unit::zero(),
            top_right_radius: Unit::zero(),
            bottom_left_radius: Unit::zero(),
            bottom_right_radius: Unit::zero(),
        }
    }
}

impl<Unit: DrawableUnit> RoundedRectangle<Unit> {
    /// Create a new RoundedRectangle.
    pub fn new(closest: impl Into<Vector>, farthest: impl Into<Vector>, radius: Unit) -> Self {
        Self {
            inner: Rectangle::new(closest, farthest),
            top_left_radius: radius.clone(),
            top_right_radius: radius.clone(),
            bottom_left_radius: radius.clone(),
            bottom_right_radius: radius,
        }
    }

    pub fn inner_rectangle(&self) -> &Rectangle<Unit> {
        &self.inner
    }

    pub fn corners_are_equal(&self) -> bool {
        self.top_left_radius == self.top_right_radius
            && self.top_right_radius == self.bottom_left_radius
            && self.bottom_left_radius == self.bottom_right_radius
    }

    pub fn corners_are_zero(&self) -> bool {
        self.top_left_radius == Unit::zero()
            && self.top_right_radius == Unit::zero()
            && self.bottom_left_radius == Unit::zero()
            && self.bottom_right_radius == Unit::zero()
    }

    pub fn translate(&mut self, point: Vector) {
        self.inner.translate(point);
    }

    pub fn closest_raw(&self) -> &Vector {
        self.inner.top_left_raw()
    }
    pub fn closest(&self) -> Vector {
        self.inner.top_left()
    }

    pub fn closest_mut(&mut self) -> &mut Vector {
        self.inner.closest_mut()
    }

    /// up to right -> start to end
    pub fn closest_quad(&self) -> QuadraticCurve<Unit> {
        self.rotate_component(self.closest_quad_raw())
    }

    /// up to right -> start to end
    pub fn closest_quad_raw(&self) -> QuadraticCurve<Unit> {
        let control = self.closest_raw().clone();
        let start = control.clone() + Vector::new(Unit::zero(), self.top_left_radius.clone());
        let end = control.clone() + Vector::new(self.top_left_radius.clone(), Unit::zero());

        QuadraticCurve::new(start, end, control)
    }

    pub fn farthest_raw(&self) -> &Vector {
        self.inner.bottom_right_raw()
    }
    pub fn farthest(&self) -> Vector {
        self.inner.bottom_right()
    }

    pub fn farthest_mut(&mut self) -> &mut Vector {
        self.inner.farthest_mut()
    }

    /// down to left -> start to end
    pub fn farthest_quad(&self) -> QuadraticCurve<Unit> {
        self.rotate_component(self.farthest_quad_raw())
    }

    /// down to left -> start to end
    pub fn farthest_quad_raw(&self) -> QuadraticCurve<Unit> {
        let control = self.farthest_raw().clone();
        let start = control.clone() + Vector::new(Unit::zero(), -self.bottom_right_radius.clone());
        let end = control.clone() + Vector::new(-self.bottom_right_radius.clone(), Unit::zero());

        QuadraticCurve::new(start, end, control)
    }

    pub fn far_close_raw(&self) -> Vec2<&Unit> {
        self.inner.top_right_raw()
    }
    pub fn far_close(&self) -> Vector {
        self.inner.top_right()
    }
    /// right to down -> start to end
    pub fn far_close_quad(&self) -> QuadraticCurve<Unit> {
        self.rotate_component(self.far_close_quad_raw())
    }

    /// right to down -> start to end
    pub fn far_close_quad_raw(&self) -> QuadraticCurve<Unit> {
        let control = self.far_close_raw();
        let control = Vector::new(control.x.clone(), control.y.clone());
        let start = control.clone() + Vector::new(-self.top_right_radius.clone(), Unit::zero());
        let end = control.clone() + Vector::new(Unit::zero(), self.top_right_radius.clone());

        QuadraticCurve::new(start, end, control)
    }

    pub fn close_far_raw(&self) -> Vec2<&Unit> {
        self.inner.bottom_left_raw()
    }
    pub fn close_far(&self) -> Vector {
        self.inner.bottom_left()
    }

    /// left to up -> start to end
    pub fn close_far_quad(&self) -> QuadraticCurve<Unit> {
        self.rotate_component(self.close_far_quad_raw())
    }

    /// left to up -> start to end
    pub fn close_far_quad_raw(&self) -> QuadraticCurve<Unit> {
        let control = self.close_far_raw();
        let control = Vector::new(control.x.clone(), control.y.clone());
        let start = control.clone() + Vector::new(self.bottom_left_radius.clone(), Unit::zero());
        let end = control.clone() + Vector::new(Unit::zero(), -self.bottom_left_radius.clone());

        QuadraticCurve::new(start, end, control)
    }

    pub fn rotation(&self) -> &Rotation {
        self.inner.rotation()
    }

    pub fn absolute_center(&self) -> Vector {
        self.inner.absolute_center()
    }

    fn rotate_component<T: Transformation<Unit>>(&self, mut component: T) -> T {
        if self.rotation().is_zero() {
            return component;
        }
        component.rotate_around(self.rotation(), &self.absolute_center());
        component
    }
}

impl<Unit: DrawableUnit> DrawableCommand for RoundedRectangle<Unit> {
    type DrawUnit = Unit;
    fn into_draw_command(self) -> DrawCommand<Self::DrawUnit> {
        if self.corners_are_zero() {
            return self.into_inner_rectangle().into_draw_command();
        }

        // first draw the top_left corner
        // before the beginning of the curve on the top left corner, so slightly below the top left corner if the radius is not 0 and is not rotated

        // draw the top left corner to the end of the curve, which is slightly right of the top left corner
        let top_left_quad = self.closest_quad();

        let tlqi = top_left_quad;

        let top_right_quad = self.far_close_quad();

        // draw from the end of the top left corner end (slightly right of the top left corner)
        // to the start of the top right corner (slightly left of the top right corner)

        // draw the top right corner to the end of the curve, which is slightly below the top right corner

        let trqs = PathCommand::line_to(top_right_quad.start.clone());

        let trqi = top_right_quad.into_bezier_chain_link();

        let bottom_right_quad = self.farthest_quad();

        // draw from the end of the top right corner end (slightly below the top right corner)
        // to the start of the bottom right corner (slightly above the bottom right corner)

        let brqs = PathCommand::line_to(bottom_right_quad.start.clone());

        // draw the bottom right corner to the end of the curve, which is slightly left of the bottom right corner
        //drawing.quadratic_bezier_curve_to(bottom_right_quad.into_bezier_chain_link());

        let brqi = bottom_right_quad.into_bezier_chain_link();

        let bottom_left_quad = self.close_far_quad();
        // draw from the end of the bottom right corner end (slightly left of the bottom right corner)
        // to the start of the bottom left corner (slightly right of the bottom left corner)

        let blqs = PathCommand::line_to(bottom_left_quad.start.clone());

        // draw the bottom left corner to the end of the curve, which is slightly above the bottom left corner
        let blqi = bottom_left_quad.into_bezier_chain_link();

        // draw from the end of the bottom left corner end (slightly above the bottom left corner)
        // to the start of the top left corner (slightly below the top left corner)

        // There will always be 9 commands in the path for rounded rectangles.
        let mut path_commands = PathCommands::with_capacity(9);

        path_commands
            .chain(tlqi)
            .add(trqs)
            .add(trqi)
            .add(brqs)
            .add(brqi)
            .add(blqs)
            .add(blqi)
            .add(PathCommand::line_to(self.closest_quad().start.clone()));

        DrawCommand::path(path_commands)
    }
}

impl<Unit: Clone + Sub<Output = Unit>> Space<Unit> for RoundedRectangle<Unit> {
    fn height(&self) -> Unit {
        self.inner.height()
    }
    fn width(&self) -> Unit {
        self.inner.width()
    }
    fn dimensions(&self) -> Dimensions<Unit> {
        self.inner.dimensions()
    }
}
