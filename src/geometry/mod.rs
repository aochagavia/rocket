// #[macro_use] needs to go first so the macro is visible for the other modules
#[macro_use]
mod vector;

mod point;
mod size;
mod traits;

pub use self::vector::Vector;
pub use self::point::Point;
pub use self::size::Size;
pub use self::traits::{Position, Advance, Collide};