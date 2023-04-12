use iced::{
    canvas::{Cursor, Frame, Geometry, LineCap, LineJoin, Path, Program, Stroke},
    Color, Point, Rectangle,
};

#[derive(Debug)]
pub(crate) struct Separator;

impl<Message> Program<Message> for Separator {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let mut frame = Frame::new(bounds.size());

        let half_height = bounds.height / 2f32;

        let line = Path::line(Point::new(0f32, half_height), Point::new(bounds.width, half_height));

        frame.stroke(&line, Stroke {
            color:     Color::from_rgb(0.82f32, 0.82f32, 0.82f32),
            width:     1f32,
            line_join: LineJoin::Miter,
            line_cap:  LineCap::Butt,
        });

        let line = Path::line(
            Point::new(0f32, half_height + 1f32),
            Point::new(bounds.width, half_height + 1f32),
        );

        frame.stroke(&line, Stroke {
            color:     Color::from_rgb(0.98f32, 0.98f32, 0.98f32),
            width:     1f32,
            line_join: LineJoin::Miter,
            line_cap:  LineCap::Butt,
        });

        vec![frame.into_geometry()]
    }
}
