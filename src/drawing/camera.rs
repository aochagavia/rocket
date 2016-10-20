use super::{Point, Size};

pub struct Camera {
    /// The size of the camera in pixels
    ///
    /// This is the same as the size of the window
    pub size: Size,
    /// The upper-left corner of the camera
    pub pos: Point
}

impl Camera {
    pub fn follow(&mut self, point: Point) {
        self.pos.x = point.x - self.size.width / 2.0;
        self.pos.y = point.y - self.size.height / 2.0;
    }
}