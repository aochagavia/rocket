/// Keeps track of active actions (toggled by user input)
#[derive(Default)]
pub struct InputController {
    pub rotate_left: bool,
    pub rotate_right: bool,
    pub boost: bool,
    pub shoot: bool,
}
