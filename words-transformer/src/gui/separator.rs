use iced::{
    mouse,
    widget::canvas::{Frame, Geometry, LineCap, LineJoin, Path, Program, Stroke, Style},
    Color, Point, Rectangle, Renderer, Theme,
};

#[derive(Debug)]
pub(crate) struct Separator;

impl<Message> Program<Message> for Separator {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());

        let half_height = bounds.height / 2f32;

        let line = Path::line(Point::new(0f32, half_height), Point::new(bounds.width, half_height));

        frame.stroke(&line, Stroke {
            style: Style::Solid(Color::from_rgb(0.82f32, 0.82f32, 0.82f32)),
            width: 1f32,
            line_join: LineJoin::Miter,
            line_cap: LineCap::Butt,
            ..Stroke::default()
        });

        let line = Path::line(
            Point::new(0f32, half_height + 1f32),
            Point::new(bounds.width, half_height + 1f32),
        );

        frame.stroke(&line, Stroke {
            style: Style::Solid(Color::from_rgb(0.98f32, 0.98f32, 0.98f32)),
            width: 1f32,
            line_join: LineJoin::Miter,
            line_cap: LineCap::Butt,
            ..Stroke::default()
        });

        vec![frame.into_geometry()]
    }
}
