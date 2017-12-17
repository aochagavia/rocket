use ggez::{Context};
use ggez::graphics;

/// Additional resources needed for the game
pub struct Resources {
    pub font: graphics::Font,
}

impl Resources {
    /// Initialize and return the `Resources`
    pub fn new(ctx: &mut Context) -> Resources {
      let font = graphics::Font::new(ctx, "/FiraMono-Bold.ttf", 14).unwrap();
      Resources { font: font }
    }
}
