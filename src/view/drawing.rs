pub mod color {
    use ggez::graphics::Color;

    pub const WHITE: Color = Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };
    pub const GREY: Color = Color {
        r: 0.6,
        g: 0.6,
        b: 0.6,
        a: 1.0,
    };
    pub const BLACK: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    pub const RED: Color = Color {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    pub const ORANGE: Color = Color {
        r: 1.0,
        g: 0.5,
        b: 0.0,
        a: 1.0,
    };
    pub const YELLOW: Color = Color {
        r: 1.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };
    pub const CYAN: Color = Color {
        r: 0.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };
    pub const GREEN: Color = Color {
        r: 0.0,
        g: 0.7,
        b: 0.0,
        a: 1.0,
    };
    pub const VIOLET: Color = Color {
        r: 0.6,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    };

    pub const SCORE: Color = VIOLET;
    pub const STAR: Color = GREY;
    pub const PARTICLE: Color = ORANGE;
    pub const BULLET: Color = CYAN;
    pub const ENEMY: Color = YELLOW;
    pub const POWERUP: Color = GREEN;
    pub const SHIELD: Color = Color {
        r: 0.0,
        g: 1.0,
        b: 1.0,
        a: 0.3,
    };
    pub const PLAYER: Color = RED;
}
